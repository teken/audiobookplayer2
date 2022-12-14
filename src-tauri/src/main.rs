#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::anyhow;
use regex::Regex;
use rodio::{OutputStream, Sink};
use serde::{Deserialize, Serialize};
// use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::PathBuf};
use surrealdb::{
    sql::{Object, Value},
    Datastore, Response, Session,
};
use tauri::Manager;
use lofty::{read_from_path, AudioFile};
use std::time::Duration;

const LIBRARY_LOCATION: &str = r"G:\Audio\Spooken Word";
const AUDIO_FILE_EXTENSIONS: [&str; 4] = ["mp4", "mp3", "m4b", "wav"];
const IMAGE_FILE_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];

struct AppState {
    // settings: Settings,
    sink: Sink,
}

// #[derive(Default, Debug, Serialize, Deserialize, Clone)]
// struct Settings {
//     library_location: String,
// }

fn main() {
    env_logger::init();

    // let settings: Settings = confy::load("audiobookplayer", "config").unwrap();

    // confy::store("audiobookplayer", "config", settings.clone()).unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(AppState { sink })
        .invoke_handler(tauri::generate_handler![
            load,
            play,
            pause,
            scan,
            search,
            start_book,
            stop,
            clear,
            close_splashscreen,
            load_work,
            library_stats,
            load_work_metadata,
        ])
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
async fn load() -> Result<Vec<Work>, LoadWorksError> {
    let (ds, ses) = &get_db().await;

    let result = ds
        .execute("SELECT * FROM works FETCH author", ses, None, false)
        .await;

    if let Err(_) = result {
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
    let series = if series_str.len() > 0 {
        Some(series_str)
    } else {
        None
    };

    let files: Vec<String> = if let Value::Array(sub_object) = object.get("files").unwrap() {
        sub_object
            .clone()
            .into_iter()
            .map(|x| x.clone().as_string())
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
        series: series,
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
fn start_book(state: tauri::State<AppState>, work_id: String) {
    // let files = fs::read_dir(book.path).unwrap();

    // let audio_files = get_audio_files(files);
    // for audio_file in audio_files {
    //     let file = BufReader::new(File::open(audio_file.path()).unwrap());
    //     let source = Decoder::new(file).unwrap();
    //     state.sink.append(source);
    // }
}

#[tauri::command]
fn play(state: tauri::State<AppState>) {
    if state.sink.is_paused() {
        state.sink.play();
    }
}

#[tauri::command]
fn pause(state: tauri::State<AppState>) {
    if !state.sink.is_paused() {
        state.sink.pause();
    }
}

#[tauri::command]
fn stop(state: tauri::State<AppState>) {
    state.sink.stop();
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
async fn scan() {
    let (ds, ses) = &get_db().await;

    println!("library scanning");
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
            println!("Failed to add work: {err}")
        }
    }

    println!("library scanned ans saved");
}

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
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
async fn load_work(work_id: String) -> Result<Work, LoadWorksError>{
    let (ds, ses) = &get_db().await;

    let ass = format!("SELECT * FROM {work_id} FETCH author");
    let result = ds
        .execute(ass.as_str(), ses, None, false)
        .await;

    if let Err(_) = result {
        return Err(LoadWorksError);
    }
    
    let mut objects = into_iter_objects(result.unwrap()).unwrap();
    
    let item = objects.nth(0).transpose();
    
    if item.is_err() {
        return Err(LoadWorksError);
    }
    
    let work = object_into_work(item.unwrap().unwrap());
    
    Ok(work)
}

#[tauri::command]
async fn load_work_metadata(work_id: String) -> Result<Vec<TrackMetadata>,ReadFileMetadataError>{
    let work = load_work(work_id).await.unwrap();
    
    let files: Vec<TrackMetadata> = work.audio_files.iter().map(|path:&String| read_file_metadata(path.clone()).unwrap()).collect::<Vec<TrackMetadata>>();
    
    println!("{files:?}");
    
    Ok(files)
}

#[derive(Debug, Serialize, Deserialize)]
struct ReadFileMetadataError {}

fn read_file_metadata(path: String) -> Result<TrackMetadata, ReadFileMetadataError> {
    let i = read_from_path(path.clone()).unwrap();
    
    let tag = i.primary_tag().unwrap();

    let metadata = TrackMetadata {
        path: path,
        duration: i.properties().duration(),
        track_title: tag.get_string(&lofty::ItemKey::TrackTitle).unwrap().to_owned(),
        track_author: tag.get_string(&lofty::ItemKey::TrackArtist).unwrap().to_owned(),
        album_title: tag.get_string(&lofty::ItemKey::AlbumTitle).unwrap().to_owned(),
    };
    
    Ok(metadata)
}

#[derive(Debug, Serialize, Deserialize)]
struct TrackMetadata {
    path:String,
    track_title:String,
    track_author:String,
    album_title:String,
    duration:Duration,
}

