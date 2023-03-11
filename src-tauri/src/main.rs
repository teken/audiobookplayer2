#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::anyhow;
use lofty::{read_from_path, AudioFile};
use log::{error, info};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::{collections::BTreeMap, fs, path::PathBuf};
use surrealdb::{
    sql::{Object, Value},
    Datastore, Response, Session,
};
use tauri::Manager;
use tauri_plugin_store::{Store, StoreBuilder};
use window_shadows::set_shadow;

const LIBRARY_LOCATION: &str = r"G:\Audio\Spooken Word";
const AUDIO_FILE_EXTENSIONS: [&str; 5] = ["mp4", "mp3", "m4a", "m4b", "wav"];
const AUDIO_FILE_WITH_CHAPTERS_EXTENSIONS: [&str; 3] = ["mp4", "m4a", "m4b"];
const IMAGE_FILE_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];

struct AppState {
    settings: Store,
}

fn main() {
    env_logger::init();

    let settings = StoreBuilder::new("./settings".parse().unwrap())
        .default("the-key".to_string(), "wooooot".into())
        .build();

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::PluginBuilder::default().build())
        .manage(AppState { settings })
        .invoke_handler(tauri::generate_handler![
            load_library,
            play,
            pause,
            scan_folder,
            scan_metadata,
            search,
            start_book,
            stop,
            clear,
            close_splashscreen,
            load_work,
            library_stats,
            load_work_metadata,
            load_book_time,
        ])
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            set_shadow(&main_window, true).expect("Unsupported platform!");

            let splashscreen = app.get_window("splashscreen").unwrap();
            set_shadow(&splashscreen, true).expect("Unsupported platform!");

            let background_player = app.get_window("background-player").unwrap();
            // background_player.show().unwrap();

            let i = app.app_handle();
            main_window.on_window_event(move |event| match event {
                tauri::WindowEvent::Destroyed => {
                    background_player
                        .close()
                        .expect("Failed to close background_player");
                    i.exit(0);
                }
                _ => {}
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn get_db() -> (Datastore, Session) {
    (
        Datastore::new("file://../data.db").await.unwrap(),
        Session::for_db("abp", "local"),
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct ClearDatabaseError;

#[tauri::command]
async fn clear() -> Result<(), ClearDatabaseError> {
    let (ds, ses) = &get_db().await;
    match ds.execute("REMOVE TABLE works", ses, None, false).await {
        Ok(_) => Ok(()),
        Err(_) => Err(ClearDatabaseError),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LoadWorksError;

#[tauri::command]
async fn load_library() -> Result<Vec<Work>, LoadWorksError> {
    let (ds, ses) = &get_db().await;

    let result = ds
        .execute("SELECT * FROM works FETCH author", ses, None, false)
        .await;

    if result.is_err() {
        return Err(LoadWorksError);
    }

    let objects = into_iter_objects(result.unwrap()).unwrap();

    Ok(objects
        .into_iter()
        .map(|object| object_into_work(object.unwrap()))
        .collect())
}

fn object_into_work(object: Object) -> Work {
    let series_str = object.get("series").map(|x| x.clone().as_string()).unwrap();
    let series = if series_str.is_empty() {
        None
    } else {
        Some(series_str)
    };

    let files: Vec<String> = if let Value::Array(sub_object) = object.get("files").unwrap() {
        sub_object
            .clone()
            .into_iter()
            .map(|x| x.as_string())
            .collect()
    } else {
        vec![]
    };
    Work {
        id: object.get("id").map(|x| x.clone().as_string()).unwrap(),
        name: object.get("name").map(|x| x.clone().as_string()).unwrap(),
        author: if let Value::Object(sub_object) = object.get("author").unwrap() {
            sub_object
                .get("name")
                .map(|x| x.clone().as_string())
                .unwrap()
        } else {
            "".to_string()
        },
        series,
        path: object.get("path").map(|x| x.clone().as_string()).unwrap(),
        files: files.clone(),
        audio_files: get_files_by_extension(files.clone(), AUDIO_FILE_EXTENSIONS.to_vec()),
        image_files: get_files_by_extension(files, IMAGE_FILE_EXTENSIONS.to_vec()),
    }
}

fn into_iter_objects(
    result: Vec<Response>,
) -> Result<impl Iterator<Item = Result<Object, String>>, String> {
    let resp = result.into_iter().next().map(|rp| rp.result).transpose()?;
    match resp {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err("record was not an object".to_owned()),
            });
            Ok(it)
        }
        _ => Err("Failed to find array in result".to_owned()),
    }
}

#[tauri::command]
async fn search(search: String) -> Vec<Work> {
    let (ds, ses) = &get_db().await;

    let ass = format!(
        "SELECT * FROM works WHERE \
            string::lowercase(name) CONTAINS string::lowercase('{search}') \
            FETCH author"
    ); // string::lowercase(author->name) CONTAINS string::lowercase('{search}') \ COALESCE(string::lowercase(series),'') CONTAINS string::lowercase('{search}') \
    let result = ds.execute(ass.as_str(), ses, None, false).await.unwrap();

    let objects = into_iter_objects(result).unwrap();

    objects
        .into_iter()
        .map(|object| object_into_work(object.unwrap()))
        .collect()
}

#[tauri::command]
async fn start_book(app_handle: tauri::AppHandle, work_id: String) {
    let work = load_work(work_id).await.unwrap();

    app_handle
        .emit_all("work_loaded", work.clone().audio_files)
        .unwrap();

    let files: Vec<TrackMetadata> = work
        .audio_files
        .iter()
        .map(|path: &String| read_file_metadata(path.clone()).unwrap())
        .collect::<Vec<TrackMetadata>>();

    app_handle.emit_all("metadata_loaded", files).unwrap();
    app_handle.emit_all("play", ()).unwrap();
}

#[tauri::command]
fn play(app_handle: tauri::AppHandle) {
    app_handle.emit_all("play", ()).unwrap();
}

#[tauri::command]
fn pause(app_handle: tauri::AppHandle) {
    app_handle.emit_all("pause", ()).unwrap();
}

#[tauri::command]
fn stop(app_handle: tauri::AppHandle) {
    app_handle.emit_all("unload", ()).unwrap();
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
struct Work {
    id: String,
    name: String,
    author: String,
    series: Option<String>,

    path: String,
    files: Vec<String>,
    image_files: Vec<String>,
    audio_files: Vec<String>,
}

fn get_files_by_extension(files: Vec<String>, extenions: Vec<&str>) -> Vec<String> {
    files
        .iter()
        .filter(|x| {
            let path = PathBuf::from(x);
            path.extension().is_some()
                && extenions.contains(&path.extension().unwrap().to_str().unwrap())
        })
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
}

#[tauri::command]
async fn scan_folder() {
    let (ds, ses) = &get_db().await;

    info!("library scanning");
    let mut library: Vec<Work> = vec![];
    let authors = fs::read_dir(LIBRARY_LOCATION);
    for author in authors.unwrap() {
        // authors
        let au = author.unwrap();
        let apath = au.path();

        if au.file_type().unwrap().is_dir() {
            let works = fs::read_dir(apath).unwrap();
            let author_name = au.file_name().into_string().unwrap();

            let author_id = create_author(ds, ses, author_name).await.unwrap();

            for work in works {
                let wu = work.unwrap();
                let wpath = wu.path();

                let work_name = wu.file_name().into_string().unwrap();
                if wu.file_type().unwrap().is_dir() {
                    let mut subworks = fs::read_dir(wpath.clone()).unwrap();

                    let is_work_series = subworks.any(|x| x.unwrap().file_type().unwrap().is_dir());
                    let files = fs::read_dir(wpath.clone())
                        .unwrap()
                        .map(|x| x.unwrap().path().as_os_str().to_str().unwrap().to_string())
                        .collect();
                    if !is_work_series {
                        library.push(Work {
                            author: author_id.clone(),
                            series: None,
                            files,
                            name: work_name,
                            path: wpath.clone().into_os_string().into_string().unwrap(),
                            ..Default::default()
                        });
                    }

                    for subwork in fs::read_dir(wpath).unwrap() {
                        let swu = subwork.unwrap();
                        let swpath = swu.path();

                        if swu.file_type().unwrap().is_dir() {
                            let subsubworks = fs::read_dir(swpath.clone()).unwrap();

                            library.push(Work {
                                author: author_id.clone(),
                                series: Some(wu.file_name().into_string().unwrap()),
                                files: subsubworks
                                    .map(|x| {
                                        x.unwrap().path().as_os_str().to_str().unwrap().to_string()
                                    })
                                    .collect(),
                                name: swu.file_name().into_string().unwrap(),
                                path: swpath.clone().into_os_string().into_string().unwrap(),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
    }

    for work in library {
        if let Err(err) = create_work(ds, ses, work).await {
            error!("Failed to add work: {err}")
        }
    }

    info!("library scanned ans saved");
}

#[tauri::command]
fn scan_metadata() {}

fn string_to_id(item: String) -> String {
    Regex::new(r"[^a-zA-Z0-9]")
        .unwrap()
        .replace_all(item.as_str(), "_")
        .to_string()
}

async fn create_work(ds: &Datastore, ses: &Session, work: Work) -> anyhow::Result<()> {
    let ass = format!("CREATE works CONTENT {{ name: $name, author: authors:{}, series: $series, path: $path, files:$files, }}", work.author);
    let data: BTreeMap<String, Value> = BTreeMap::from([
        ("name".into(), work.name.into()),
        (
            "series".into(),
            match work.series {
                Some(v) => v,
                None => "".to_owned(),
            }
            .into(),
        ),
        ("path".into(), work.path.into()),
        (
            "files".into(),
            work.files
                .iter()
                .map(String::as_str)
                .collect::<Vec<&str>>()
                .into(),
        ),
    ]);
    match ds.execute(ass.as_str(), ses, Some(data), false).await {
        Ok(_) => anyhow::Ok(()),
        Err(err) => Err(anyhow!("failed to create work: {}", err)),
    }
}

async fn create_author(
    ds: &Datastore,
    ses: &Session,
    author_name: String,
) -> anyhow::Result<String> {
    let author_id = string_to_id(author_name.clone());
    let ass = format!(
        "CREATE authors:{} CONTENT {{ name: $name }}",
        author_id.clone()
    );
    let data: BTreeMap<String, Value> =
        BTreeMap::from([("name".into(), author_name.replace("'", r"\'").into())]);
    match ds.execute(ass.as_str(), ses, Some(data), false).await {
        Ok(_) => anyhow::Ok(author_id),
        Err(err) => Err(anyhow!("failed to create work: {}", err)),
    }
}

#[tauri::command]
async fn library_stats() {
    // let (ds, ses) = &get_db().await;

    // let ass = format!(
    //     "SELECT \
    //         (SELECT count(*) FROM works) as books, \
    //         (SELECT count(string::length(series) == 0) FROM works) as booksNotInSeries, \
    //     "
    // ); // string::lowercase(author->name) CONTAINS string::lowercase('{search}') \ COALESCE(string::lowercase(series),'') CONTAINS string::lowercase('{search}') \
    // let result = ds.execute(ass.as_str(), ses, None, false).await.unwrap();

    // let objects = into_iter_objects(result).unwrap();
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    window
        .get_window("splashscreen")
        .expect("splashscreen not found")
        .close()
        .expect("splashscreen not closed");
    window
        .get_window("main")
        .expect("main not found")
        .show()
        .expect("main not opened");
}

#[tauri::command]
async fn load_work(work_id: String) -> Result<Work, LoadWorksError> {
    let (ds, ses) = &get_db().await;

    let ass = format!("SELECT * FROM {work_id} FETCH author");
    let Ok(result) = ds.execute(ass.as_str(), ses, None, false).await else {
        return Err(LoadWorksError);
    };

    let Ok(mut objects) = into_iter_objects(result) else {
        return Err(LoadWorksError);
    };

    let Some(Ok(item)) = objects.next() else {
        return Err(LoadWorksError);
    };

    let work = object_into_work(item);

    Ok(work)
}

#[tauri::command]
async fn load_work_metadata(work_id: String) -> Result<Vec<TrackMetadata>, ReadFileMetadataError> {
    let work = load_work(work_id).await.unwrap();

    let files: Vec<TrackMetadata> = work
        .audio_files
        .iter()
        .map(|path: &String| read_file_metadata(path.clone()).unwrap())
        .collect::<Vec<TrackMetadata>>();

    Ok(files)
}

#[derive(Debug, Serialize, Deserialize)]
struct ReadFileMetadataError {}

fn read_file_metadata(path: String) -> Result<TrackMetadata, ReadFileMetadataError> {
    let file = read_from_path(path.clone()).unwrap();

    let tag = file.primary_tag().unwrap();

    let mut chapters = vec![];

    let c_path = PathBuf::from(path.clone());
    if c_path.extension().is_some()
        && AUDIO_FILE_WITH_CHAPTERS_EXTENSIONS
            .contains(&c_path.extension().unwrap().to_str().unwrap())
    {
        let f = File::open(path.clone()).unwrap();
        let size = f.metadata().unwrap().len();
        let reader = BufReader::new(f);
        let mut mp4 = mp4::Mp4Reader::read_header(reader, size).unwrap();

        let track = mp4
            .tracks()
            .values()
            .find(|x| x.media_type().is_err())
            .unwrap();
        let track_id = track.track_id();
        for i in 0..track.sample_count() {
            let sample = mp4.read_sample(track_id, i + 1).unwrap().unwrap();

            chapters.push(Chapter {
                title: format!("Chapter {}", i),
                length: if sample.duration == 0 {
                    file.properties().duration() - Duration::from_millis(sample.start_time)
                } else {
                    Duration::from_millis(sample.duration.into())
                },
            });
        }
    } else {
        chapters.push(Chapter {
            title: tag
                .get_string(&lofty::ItemKey::TrackTitle)
                .unwrap_or_default()
                .to_owned(),
            length: file.properties().duration(),
        });
    }

    let metadata = TrackMetadata {
        path,
        duration: file.properties().duration(),
        track_title: tag
            .get_string(&lofty::ItemKey::TrackTitle)
            .unwrap_or_default()
            .to_owned(),
        track_author: tag
            .get_string(&lofty::ItemKey::TrackArtist)
            .unwrap_or_default()
            .to_owned(),
        album_title: tag
            .get_string(&lofty::ItemKey::AlbumTitle)
            .unwrap_or_default()
            .to_owned(),
        chapters,
    };

    Ok(metadata)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TrackMetadata {
    path: String,
    track_title: String,
    track_author: String,
    album_title: String,
    duration: Duration,
    chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Chapter {
    title: String,
    length: Duration,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ReadWorkDataError {}

#[tauri::command]
async fn load_book_time(work_id: String) -> Result<Duration, ReadWorkDataError> {
    Ok(Duration::from_secs(23456))
}
