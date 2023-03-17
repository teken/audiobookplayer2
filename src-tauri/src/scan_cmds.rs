use log::{error, info};
use std::fs;

use crate::types::Work;
use crate::utils::{create_author, create_work, LIBRARY_LOCATION};

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
            error!("Failed to add work: {err}")
        }
    }

    info!("library scanned ans saved");
}

#[tauri::command]
pub fn scan_metadata() {
    // todo: build library by file metadata
}
