use crate::types::{ClearDatabaseError, LoadWorksError, Work};
use crate::utils::{into_iter_objects, object_into_work};

use crate::DB;
use crate::SES;

#[tauri::command]
pub async fn clear_library() -> Result<(), ClearDatabaseError> {
    DB.get()
        .await
        .execute("REMOVE TABLE works", &SES, None, false)
        .await
        .map(|_| ())
        .map_err(|_| ClearDatabaseError)
}

#[tauri::command]
pub async fn load_library() -> Result<Vec<Work>, LoadWorksError> {
    let result = DB
        .get()
        .await
        .execute("SELECT * FROM works FETCH author", &SES, None, false)
        .await
        .map_err(|_| LoadWorksError)?;

    let objects = into_iter_objects(result).map_err(|_| LoadWorksError)?;

    Ok(objects
        .into_iter()
        .map(|object| object_into_work(object.expect("should be good")))
        .collect())
}

#[tauri::command]
pub async fn search(search: String) -> Vec<Work> {
    let ass = format!(
        "SELECT * FROM works WHERE \
            string::lowercase(name) CONTAINS string::lowercase('{search}') \
            FETCH author"
    ); // string::lowercase(author->name) CONTAINS string::lowercase('{search}') \ COALESCE(string::lowercase(series),'') CONTAINS string::lowercase('{search}') \
    let result = DB
        .get()
        .await
        .execute(ass.as_str(), &SES, None, false)
        .await
        .unwrap();

    let objects = into_iter_objects(result).unwrap();

    objects
        .into_iter()
        .map(|object| object_into_work(object.unwrap()))
        .collect()
}

#[tauri::command]
pub async fn library_stats() {
    // let (ds, ses) = &get_db().await;

    // let ass = format!(
    //     "SELECT \
    //         (SELECT count(*) FROM works) as books, \
    //         (SELECT count(string::length(series) == 0) FROM works) as booksNotInSeries, \
    //     "
    // ); // string::lowercase(author->name) CONTAINS string::lowercase('{search}') \ COALESCE(string::lowercase(series),'') CONTAINS string::lowercase('{search}') \
    // let result = DB.get().await.execute(ass.as_str(), ses, None, false).await.unwrap();

    // let objects = into_iter_objects(result).unwrap();
    // todo: enable stats for about page
}

#[tauri::command]
pub async fn clear_times() -> Result<(), ClearDatabaseError> {
    DB.get()
        .await
        .execute("REMOVE TABLE times", &SES, None, false)
        .await
        .map(|_| ())
        .map_err(|_| ClearDatabaseError)
}
