#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use async_once::AsyncOnce;
use dotenv::dotenv;
use lazy_static::lazy_static;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use tauri::Manager;
use window_shadows::set_shadow;

mod book_cmds;
mod library_cmds;
mod player_cmds;
mod scan_cmds;
mod settings_cmds;
mod types;
mod utils;

lazy_static! {
    static ref DB: AsyncOnce<Datastore> = AsyncOnce::new(async {
        let db_path = std::env::var("SURREAL_PATH").unwrap_or("file://../data.db".to_owned());
        Datastore::new(db_path.as_str())
            .await
            .expect("failed to open db")
    });
    static ref SES: Session = Session::for_db("abp", "local");
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            book_cmds::clear_book_time,
            book_cmds::load_book_time,
            book_cmds::load_work_metadata,
            book_cmds::load_work,
            book_cmds::start_book,
            book_cmds::update_work_time,
            library_cmds::clear_library,
            library_cmds::clear_times,
            library_cmds::library_stats,
            library_cmds::load_library,
            library_cmds::search,
            player_cmds::pause,
            player_cmds::play,
            player_cmds::stop,
            scan_cmds::scan_folder,
            scan_cmds::scan_metadata,
            settings_cmds::load_settings,
            settings_cmds::save_settings,
            close_splashscreen,
        ])
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            set_shadow(&main_window, true).expect("Unsupported platform!");

            let splashscreen = app.get_window("splashscreen").unwrap();
            set_shadow(&splashscreen, true).expect("Unsupported platform!");

            let background_player = app.get_window("background-player").unwrap();
            background_player
                .hide()
                .expect("Failed to hide background_player");

            let i = app.app_handle();
            main_window.on_window_event(move |event| {
                if let tauri::WindowEvent::Destroyed = event {
                    background_player
                        .close()
                        .expect("Failed to close background_player");
                    i.exit(0);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    window
        .get_window("splashscreen")
        .expect("splashscreen not found")
        .close()
        .expect("splashscreen not closed");
    window
        .get_window("main")
        .expect("main not found")
        .show()
        .expect("main not opened");
}
