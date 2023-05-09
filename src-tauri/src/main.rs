#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::str::FromStr;

use dotenv::dotenv;
use log::{error, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use once_cell::sync::{Lazy, OnceCell};
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

pub static DB: OnceCell<Datastore> = OnceCell::new();
pub static SES: Lazy<Session> = Lazy::new(|| Session::for_db("abp", "local"));

#[tokio::main]
async fn main() {
    dotenv().ok();

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
            let mut log_path = app.path_resolver().app_log_dir().expect("unknown log dir");
            log_path.push("abp.log");

            let stdout = ConsoleAppender::builder().build();

            let file = FileAppender::builder()
                .append(false)
                .build(log_path.to_str().expect("Failed to convert path"))
                .expect("File appender build failed");

            let config = Config::builder()
                .appender(Appender::builder().build("stdout", Box::new(stdout)))
                .appender(Appender::builder().build("file", Box::new(file)))
                .build(
                    Root::builder().appender("stdout").appender("file").build(
                        LevelFilter::from_str(
                            &std::env::var("RUST_LOG").unwrap_or("WARN".to_string()),
                        )
                        .unwrap(),
                    ),
                )
                .expect("Failed to build log config");

            log4rs::init_config(config).expect("Failed to setup logging");

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

            let mut db_path = app
                .path_resolver()
                .app_data_dir()
                .expect("unknown data dir");
            db_path.push("db");

            let j = app.app_handle();
            tauri::async_runtime::spawn(async move {
                let store = Datastore::new(
                    format!(
                        "file://{}",
                        db_path.to_str().expect("Failed to convert path")
                    )
                    .as_str(),
                )
                .await;
                if let Err(err) = store {
                    error!("Failed to create DB: {}", err);
                    j.exit(1);
                    return;
                }

                if let Err(_) = DB.set(store.unwrap()) {
                    error!("Failed to set DB");
                    j.exit(1);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    if let Some(win) = window.get_window("splashscreen") {
        win.close().expect("splashscreen not closed");
    }
    window
        .get_window("main")
        .expect("main not found")
        .show()
        .expect("main not opened");
}
