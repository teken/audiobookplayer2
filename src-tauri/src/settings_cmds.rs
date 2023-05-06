use std::fs::File;
use std::io::Write;

use crate::types::Settings;

const SETTINGS_FILE: &str = "settings.json";

#[tauri::command]
pub async fn load_settings(app_handle: tauri::AppHandle) -> Result<Settings, String> {
    let mut path = app_handle
        .path_resolver()
        .app_config_dir()
        .expect("cannot find app config path");
    path.push(SETTINGS_FILE);
    let Ok(file) = File::open(path.clone()) else {
        return Err(format!("Could not open {:?} file", path));
    };
    let Ok(settings) = serde_json::from_reader(file) else {
        return Err(format!("Could not deserialize settings"));
    };
    Ok(settings)
}

#[tauri::command]
pub async fn save_settings(
    app_handle: tauri::AppHandle,
    new_settings: Settings,
) -> Result<(), String> {
    let mut path = app_handle
        .path_resolver()
        .app_config_dir()
        .expect("cannot find app config path");
    path.push(SETTINGS_FILE);
    let Ok(mut file) = File::create(path.clone()) else {
        return Err(format!("Could not creat {:?} file", path));
    };
    let Ok(settings) = serde_json::to_string(&new_settings) else {
        return Err(format!("Could not serialize new settings"));
    };

    if file.write_all(settings.as_bytes()).is_err() {
        return Err(format!("Could not write to {:?} file", path));
    }
    Ok(())
}
