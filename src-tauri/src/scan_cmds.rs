use lofty::TaggedFileExt;
use log::{debug, error, info};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use walkdir::WalkDir;

use crate::types::{MetadataTemplate, Work};
use crate::utils::{
    create_author, create_work, AUDIO_FILE_EXTENSIONS, IMAGE_FILE_EXTENSIONS, LIBRARY_LOCATION,
};

#[tauri::command]
pub async fn scan_folder(window: tauri::Window) {
    info!("library scanning");
    let mut library: Vec<Work> = vec![];

    window
        .emit("scan_finding_files", 0)
        .expect("event emit failed");

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

    window.emit("scan_complete", 0).expect("event emit failed");

    info!("library scanned and saved");
}

#[tauri::command]
pub async fn scan_metadata(app_handle: tauri::AppHandle, window: tauri::Window) {
    scan_metadata_with_template(MetadataTemplate::default(), app_handle, window).await;
}

fn get_tag_with_fallback<'a>(
    tag: &'a lofty::Tag,
    keys: &'a Vec<lofty::ItemKey>,
) -> Option<&'a str> {
    for key in keys {
        if let Some(value) = tag.get_string(key) {
            return Some(value);
        }
    }
    None
}

async fn scan_metadata_with_template(
    template: MetadataTemplate,
    app_handle: tauri::AppHandle,
    window: tauri::Window,
) {
    info!("library scanning");
    let mut library: HashMap<String, Work> = HashMap::new();
    let mut authors: HashMap<String, String> = HashMap::new();

    window
        .emit("scan_finding_files", 0)
        .expect("event emit failed");

    let files = WalkDir::new(LIBRARY_LOCATION)
        .max_depth(4)
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
        })
        .collect::<Vec<_>>();

    window
        .emit("scan_metadata_files_found", files.len())
        .expect("event emit failed");

    for entry in files {
        let Ok(meta) = lofty::read_from_path(entry.path()) else {
            debug!("Failed Read {:?}", entry);
            window.emit("scan_metadata_file_failed_read", entry.path().to_str().unwrap().to_string()).expect("event emit failed");
            continue;
        };

        let tag = if meta.primary_tag().is_some() {
            meta.primary_tag().unwrap()
        } else if meta.first_tag().is_some() {
            meta.first_tag().unwrap()
        } else {
            debug!("Failed Read Tag {:?}", entry);
            window
                .emit(
                    "scan_metadata_file_failed_tag_read",
                    entry.path().to_str().unwrap().to_string(),
                )
                .expect("event emit failed");
            continue;
        };

        let Some(track_author) = get_tag_with_fallback(tag, &template.author) else {
            debug!("Failed Read Author {:?}", entry);
            window.emit("scan_metadata_file_failed_author_read", entry.path().to_str().unwrap().to_string()).expect("event emit failed");
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

        let Some(album_title) = get_tag_with_fallback(tag, &template.title) else {
                debug!("Failed Read Album {:?}", entry);
                window.emit("scan_metadata_file_failed_album_read", entry.path().to_str().unwrap().to_string()).expect("event emit failed");
                continue;
            };

        let library_key = track_author.to_owned() + album_title;

        let mut work = if library.contains_key(&library_key) {
            library.remove(&library_key).unwrap()
        } else {
            let images = WalkDir::new(entry.path().parent().unwrap())
                .into_iter()
                .filter_map(|e| match e {
                    Ok(dir) => {
                        if dir.path().extension().is_some()
                            && IMAGE_FILE_EXTENSIONS
                                .contains(&dir.path().extension().unwrap().to_str().unwrap())
                        {
                            Some(dir.path().to_str().unwrap().to_string())
                        } else {
                            None
                        }
                    }
                    Err(..) => None,
                })
                .collect::<Vec<_>>();
            Work {
                author: author_id,
                series: None,
                files: images.clone(),
                image_files: images,
                name: album_title.to_owned(),
                path: entry.path().parent().unwrap().to_str().unwrap().to_string(),
                ..Default::default()
            }
        };

        if work.image_files.is_empty() {
            let mut cover_pics = tag
                .pictures()
                .iter()
                .filter(|pic| pic.pic_type() == lofty::PictureType::CoverFront);
            if cover_pics.clone().count() > 0 {
                let cover_pic = cover_pics.next().unwrap();
                let extension = match cover_pic.mime_type().as_str() {
                    "image/jpeg" => "jpg",
                    "image/png" => "png",
                    "image/gif" => "gif",
                    "image/bmp" => "bmp",
                    e => {
                        error!("missing file type support: {}", e);
                        "jpg"
                    }
                };
                let cover_path = app_handle
                    .path_resolver()
                    .app_cache_dir()
                    .unwrap()
                    .clone()
                    .join("cache_covers")
                    .join(format!(
                        "{}.{}",
                        regex::Regex::new(r"[^A-Za-z0-9 ]")
                            .unwrap()
                            .replace_all(album_title, ""),
                        extension
                    ));

                if cover_path.exists() {
                    work.files.push(cover_path.to_str().unwrap().into());
                } else {
                    if !cover_path.parent().unwrap().exists() {
                        fs::create_dir_all(cover_path.parent().unwrap()).unwrap();
                    }
                    match File::create(&cover_path) {
                        Ok(mut cover_file) => {
                            cover_file.write_all(cover_pic.data()).unwrap();
                            work.files.push(cover_path.to_str().unwrap().into());
                        }
                        Err(err) => {
                            error!("Failed to create cover file: {:?}: {:?}", cover_path, err)
                        }
                    }
                }
            }
        }

        work.files.push(entry.path().to_str().unwrap().to_string());

        library.insert(library_key.clone(), work);

        window
            .emit("scan_metadata_file_complete", 0)
            .expect("event emit failed");
    }

    for work in library.values() {
        if let Err(err) = create_work(work.clone()).await {
            error!("Failed to add work: {}", err);
        }
    }

    window.emit("scan_complete", 0).expect("event emit failed");

    info!("library scanned and saved");
}
