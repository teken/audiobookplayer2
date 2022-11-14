#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    fs::{self, DirEntry, File, ReadDir},
    io::BufReader,
    path::PathBuf,
};

use pallet::DocumentLike;
use rodio::{Decoder, OutputStream, Sink};
use tauri::async_runtime::spawn;

const LIBRARY_LOCATION: &str = r"G:\Audio\Spooken Word";
const AUDIO_FILE_EXTENSIONS: &'static [&'static str] = &["mp4", "mp3", "m4b", "wav"];
const IMAGE_FILE_EXTENSIONS: &'static [&'static str] = &["jpg", "jpeg", "png"];

struct AppState {
    sink: Sink,
    works: pallet::Store<Work>,
}

fn main() {
    env_logger::init();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let db = sled::open("./data").unwrap();
    let store: pallet::Store<Work> = pallet::Store::builder()
        .with_db(db)
        .with_index_dir("./data")
        .finish()
        .unwrap();

    tauri::Builder::default()
        .manage(AppState { sink, works: store })
        .invoke_handler(tauri::generate_handler![
            load, play, pause, scan, search, start_book, stop,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load(state: tauri::State<AppState>) -> Vec<Work> {
    state
        .works
        .all()
        .unwrap()
        .iter()
        .map(|x| x.inner.clone())
        .collect()
}

#[tauri::command]
fn search(state: tauri::State<AppState>, search: String) -> Vec<Work> {
    let mut search_result = state.works.search(search.as_str()).unwrap().hits;
    search_result.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
    search_result.iter().map(|x| x.doc.inner.clone()).collect()
}

#[tauri::command(async)]
fn start_book(state: tauri::State<AppState>, book: Work) {
    let files = fs::read_dir(book.path).unwrap();

    let audio_files = get_audio_files(files);
    for audio_file in audio_files {
        let file = BufReader::new(File::open(audio_file.path()).unwrap());
        let source = Decoder::new(file).unwrap();
        state.sink.append(source);
    }
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default, DocumentLike)]
#[pallet(tree_name = "works")]
struct Work {
    #[pallet(default_search_field)]
    name: String,
    #[pallet(default_search_field)]
    author: String,
    #[pallet(default_search_field)]
    series: Option<String>,

    path: String,
}

fn get_audio_files(files: ReadDir) -> Vec<DirEntry> {
    files
        .into_iter()
        .filter(|x| {
            let dir_entry = x.as_ref().unwrap();
            dir_entry.path().extension().is_some()
                && AUDIO_FILE_EXTENSIONS
                    .contains(&dir_entry.path().extension().unwrap().to_str().unwrap())
        })
        .map(|x| x.unwrap())
        .collect()
}

fn get_image_files(files: ReadDir) -> Vec<DirEntry> {
    files
        .into_iter()
        .filter(|x| {
            let dir_entry = x.as_ref().unwrap();
            dir_entry.path().extension().is_some()
                && IMAGE_FILE_EXTENSIONS
                    .contains(&dir_entry.path().extension().unwrap().to_str().unwrap())
        })
        .map(|x| x.unwrap())
        .collect()
}

#[tauri::command(async)]
fn scan(state: tauri::State<AppState>) {
    println!("library scanning");
    let mut library: Vec<Work> = vec![];
    let authors = fs::read_dir(LIBRARY_LOCATION);
    for author in authors.unwrap() {
        // authors
        let au = author.unwrap();
        let apath = au.path();

        if au.file_type().unwrap().is_dir() {
            let works = fs::read_dir(apath).unwrap();

            for work in works {
                let author_name = au.file_name().into_string().unwrap();
                let wu = work.unwrap();
                let wpath = wu.path();

                let work_name = wu.file_name().into_string().unwrap();
                if wu.file_type().unwrap().is_dir() {
                    let mut subworks = fs::read_dir(wpath.clone()).unwrap();

                    let is_work_series = subworks.any(|x| x.unwrap().file_type().unwrap().is_dir());
                    // let files = fs::read_dir(wpath.clone())
                    //     .unwrap()
                    //     .map(|x| x.unwrap().path().as_os_str().to_str().unwrap().to_string())
                    //     .collect();
                    if !is_work_series {
                        library.push(Work {
                            author: author_name,
                            series: None,
                            // files,
                            name: work_name,
                            path: wpath.clone().into_os_string().into_string().unwrap(),
                        });
                    }

                    for subwork in fs::read_dir(wpath).unwrap() {
                        let swu = subwork.unwrap();
                        let swpath = swu.path();

                        if swu.file_type().unwrap().is_dir() {
                            let subsubworks = fs::read_dir(swpath.clone()).unwrap();

                            library.push(Work {
                                author: au.file_name().into_string().unwrap(),
                                series: Some(wu.file_name().into_string().unwrap()),
                                // files: subsubworks
                                //     .map(|x| {
                                //         x.unwrap().path().as_os_str().to_str().unwrap().to_string()
                                //     })
                                //     .collect(),
                                name: swu.file_name().into_string().unwrap(),
                                path: swpath.clone().into_os_string().into_string().unwrap(),
                            });
                        }
                    }
                }
            }
        }
    }

    state.works.create_multi(library.as_slice());
    // for ele in library {
    //     if let Err(e) = state.works.create_multi(ele) {
    //         println!("Failed to add to library {:?} : {:?}", ele, e)
    //     }
    // }
    println!("library scanned ans saved");
}
