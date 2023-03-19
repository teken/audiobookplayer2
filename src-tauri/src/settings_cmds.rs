use std::fs::File;
use std::io::Write;

use crate::types::Settings;

const SETTINGS_FILE: &str = "settings.json";

#[tauri::command]
pub async fn load_settings() -> Result<Settings, String> {
    let Ok(file) = File::open(SETTINGS_FILE) else {
        return Err(format!("Could not open {} file", SETTINGS_FILE));
    };
    let Ok(settings) = serde_json::from_reader(file) else {
        return Err(format!("Could not deserialize settings"));
    };
    Ok(settings)
}

#[tauri::command]
pub async fn save_settings(new_settings: Settings) -> Result<(), String> {
    let Ok(mut file) = File::create(SETTINGS_FILE) else {
        return Err(format!("Could not creat {} file", SETTINGS_FILE));
    };
    let Ok(settings) = serde_json::to_string(&new_settings) else {
        return Err(format!("Could not serialize new settings"));
    };

    if file.write_all(settings.as_bytes()).is_err() {
        return Err(format!("Could not write to {} file", SETTINGS_FILE));
    }
    Ok(())
}
