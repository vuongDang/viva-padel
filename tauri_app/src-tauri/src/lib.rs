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

use tauri::Manager;
use tauri_plugin_store::StoreExt;

pub(crate) const FILTERS_STORE: &str = "filters.json";
pub(crate) const FILTERS_KEY: &str = "filters";
pub(crate) const DEFAULT_FILTER_KEY: &str = "default_filter";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            // Load the store
            let _filters_store = app.store(FILTERS_STORE)?;

            // Open devtools
            let window = app.get_webview_window("main").unwrap();
            window.open_devtools();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            save_filters,
            set_default_filter,
            get_date_planning,
            get_stored_filters,
            get_default_filter,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
