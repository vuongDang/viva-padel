#[cfg(not(feature = "local_dev"))]
mod server_calls;
#[cfg(not(feature = "local_dev"))]
use server_calls::*;

#[cfg(feature = "local_dev")]
mod local_dev_server_calls;
#[cfg(feature = "local_dev")]
use local_dev_server_calls::*;
mod filters_commands;
use filters_commands::*;

use shared::frontend::calendar_ui::Filter;
use std::{collections::HashMap, sync::Mutex};
use tauri::Manager;
use tauri_plugin_store::StoreExt;

pub(crate) const FILTERS_STORE: &str = "filters.json";
pub(crate) const FILTERS_KEY: &str = "filters";

struct AppData {
    pub(crate) filters: HashMap<String, Filter>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            // Load the store
            let filters_store = app.store(FILTERS_STORE)?;

            // Open devtools
            let window = app.get_webview_window("main").unwrap();
            window.open_devtools();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            save_filters,
            get_date_planning,
            get_stored_filters
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
