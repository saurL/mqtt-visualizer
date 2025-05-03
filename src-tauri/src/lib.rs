// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod app;
use app::App;
use std::{env, sync::Arc};
use tauri::{Manager, State};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level_for("paho_mqtt", log::LevelFilter::Off)
                .level_for("paho_mqtt_c", log::LevelFilter::Off)
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .build(),
        )
        .setup(|tauri_app| {
            // or perform any other setup tasks.
            tauri_app.manage(App::new(tauri_app.handle().clone()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_data_names])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_data_names(app: State<Arc<App>>) -> Vec<String> {
    app.get_data()
}
