use tauri::Manager;

#[tauri::command]
pub fn play(app_handle: tauri::AppHandle) {
    app_handle.emit_all("play", ()).unwrap();
}

#[tauri::command]
pub fn pause(app_handle: tauri::AppHandle) {
    app_handle.emit_all("pause", ()).unwrap();
}

#[tauri::command]
pub fn stop(app_handle: tauri::AppHandle) {
    app_handle.emit_all("unload", ()).unwrap();
}
