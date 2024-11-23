use serde_json::json;
use tauri_plugin_store::StoreExt;
use std::collections::HashMap;
use shared::frontend::calendar_ui::Filter;
use shared::errors::Error;
use crate::{FILTERS_STORE, FILTERS_KEY};


#[tauri::command]
pub(crate) async fn save_filters(app_handle: tauri::AppHandle,  filters: HashMap<String, Filter>) -> Result<(), Error> {
    // Save the new filters to disk
    if let Some(store) = app_handle.get_store(FILTERS_STORE) {
        store.set(FILTERS_KEY, json!(filters));
        store.save().map_err(|e| Error::StoreError(e.to_string()))?;
        Ok(())
    } else {
        Err(Error::StoreError("Store not loaded".to_string()))
    }
}

#[tauri::command]
pub(crate) async fn get_stored_filters(app_handle: tauri::AppHandle) -> Result<HashMap<String, Filter>, Error> {
    if let Some(store) = app_handle.get_store(FILTERS_STORE) {
        if let Some(store_json) = store.get(FILTERS_KEY) {
            serde_json::from_value(store_json)?
        } else {
            Ok(Filter::default_filters())
        }
    } else {
        Err(Error::StoreError("Store not loaded".to_string()))
    }
}
