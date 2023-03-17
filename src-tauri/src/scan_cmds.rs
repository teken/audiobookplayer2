use lofty::TaggedFileExt;
use log::{error, info};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

use crate::types::Work;
use crate::utils::{create_author, create_work, AUDIO_FILE_EXTENSIONS, LIBRARY_LOCATION};

#[tauri::command]
pub async fn scan_folder() {
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

            let author_id = create_author(author_name).await.unwrap();

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
        if let Err(err) = create_work(work).await {
            error!("Failed to add work: {}", err);
        }
    }

    info!("library scanned and saved");
}

#[tauri::command]
pub async fn scan_metadata(window: tauri::Window) {

    info!("library scanning");
    let mut library: HashMap<String, Work> = HashMap::new();
    let mut authors: HashMap<String, String> = HashMap::new();

    window.emit("scan_metadata_finding_files", 0);

    let files = WalkDir::new(LIBRARY_LOCATION)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| match e {
            Ok(dir) => {
                if dir.path().extension().is_some()
                    && AUDIO_FILE_EXTENSIONS
                        .contains(&dir.path().extension().unwrap().to_str().unwrap())
                {
                    Some(dir)
                } else {
                    None
                }
            }
            Err(..) => None,
        }).collect::<Vec<_>>();

    window.emit("scan_metadata_files_found", files.len());

    for entry in files {
        let Ok(meta) = lofty::read_from_path(entry.path()) else {
            error!("Failed {:?}", entry);
            window.emit("scan_metadata_file_failed_read", entry.path());
            continue;
        };

        let Some(tag) = meta.primary_tag() else {
            error!("Failed Tag {:?}", entry);
            window.emit("scan_metadata_file_failed_tag_read", entry.path());
            continue;
        };

        let Some(track_author) = tag.get_string(&lofty::ItemKey::TrackArtist) else {
            error!("Failed Author {:?}", entry);
            window.emit("scan_metadata_file_failed_author_read", entry.path());
            continue;
        };

        let author_id = if authors.contains_key(track_author) {
            authors.get(track_author).unwrap().to_owned()
        } else {
            let key = create_author(track_author.clone().to_owned())
                .await
                .unwrap();
            authors.insert(track_author.clone().to_owned(), key.clone());
            key
        };

        let Some(album_title) = tag
            .get_string(&lofty::ItemKey::AlbumTitle) else {
                error!("Failed Album {:?}", entry);
                window.emit("scan_metadata_file_failed_album_read", entry.path());
                continue;
            };

        let library_key = track_author.to_owned() + album_title;

        let mut work = if library.contains_key(&library_key) {
            library.remove(&library_key).unwrap()
        } else {
            Work {
                author: author_id,
                series: None,
                files: vec![],
                name: album_title.to_owned(),
                path: entry.path().to_str().unwrap().to_string(),
                ..Default::default()
            }
        };

        work.files.push(entry.path().to_str().unwrap().to_string());

        library.insert(library_key.clone(), work);

        window.emit("scan_metadata_file_complete", 0);
    }



    for work in library.values() {
        if let Err(err) = create_work(work.clone()).await {
            error!("Failed to add work: {}", err);
        }
    }

    window.emit("scan_metadata_complete", 0);

    info!("library scanned and saved");
}
