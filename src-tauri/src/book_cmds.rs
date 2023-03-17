use log::error;
use tauri::Manager;

use crate::types::{
    AddWorkTimeError, LoadWorksError, ReadFileMetadataError, ReadWorkDataError, TrackMetadata, Work,
};
use crate::utils::{into_iter_objects, object_into_work, read_file_metadata};
use crate::DB;
use crate::SES;

#[tauri::command]
pub async fn start_book(app_handle: tauri::AppHandle, work_id: String) {
    let work = load_work(work_id).await.unwrap();

    app_handle.emit_all("work_loaded", work.clone()).unwrap();

    let files: Vec<TrackMetadata> = work
        .audio_files
        .iter()
        .map(|path: &String| read_file_metadata(path.clone()).unwrap())
        .collect::<Vec<TrackMetadata>>();

    app_handle.emit_all("metadata_loaded", files).unwrap();
    app_handle.emit_all("play", ()).unwrap();
}

#[tauri::command]
pub async fn load_book_time(work_id: String) -> Result<Option<f64>, ReadWorkDataError> {
    let ass = format!("SELECT position FROM times WHERE work={}", work_id);
    let Ok(result) = DB.get().await
        .execute(ass.as_str(), &SES, None, false)
        .await else {
            return Err(ReadWorkDataError {});
        };

    let Ok(objects) = into_iter_objects(result) else {
        return Err(ReadWorkDataError {});
    };

    let Some(Ok(obj)) = objects.last() else {
        return Ok(None);
    };

    let dur = obj
        .get("position")
        .map(|x| x.clone().as_float())
        .unwrap_or_default();

    Ok(Some(dur))
}

#[tauri::command]
pub async fn load_work(work_id: String) -> Result<Work, LoadWorksError> {
    let ass = format!("SELECT * FROM {work_id} FETCH author");
    let Ok(result) = DB.get().await.execute(ass.as_str(), &SES, None, false).await else {
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
pub async fn load_work_metadata(
    work_id: String,
) -> Result<Vec<TrackMetadata>, ReadFileMetadataError> {
    let work = load_work(work_id).await.unwrap();

    let files: Vec<TrackMetadata> = work
        .audio_files
        .iter()
        .map(|path: &String| read_file_metadata(path.clone()).unwrap())
        .collect::<Vec<TrackMetadata>>();

    Ok(files)
}

#[tauri::command]
pub async fn update_work_time(work_id: String, position: f64) -> Result<(), AddWorkTimeError> {
    // todo: fix times not updating on duplicate keys
    let ass = format!(
        "INSERT INTO times (work, position) VALUES ({},{}) ON DUPLICATE KEY UPDATE position={};",
        work_id, position, position
    );
    match DB
        .get()
        .await
        .execute(ass.as_str(), &SES, None, false)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("{:?}", err);
            Err(AddWorkTimeError)
        }
    }
}

#[tauri::command]
pub async fn clear_book_time(work_id: String) -> Result<(), String> {
    let ass = format!("DELETE times WHERE work={}", work_id);
    match DB
        .get()
        .await
        .execute(ass.as_str(), &SES, None, false)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("{:?}", err);
            Err("failed to clear book time".into())
        }
    }
}
