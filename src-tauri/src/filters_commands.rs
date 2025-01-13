use crate::{DEFAULT_FILTER_KEY, FILTERS_KEY, FILTERS_STORE};
use serde_json::json;
use shared::errors::Error;
use shared::filter::Filter;
use std::collections::HashMap;
use tauri_plugin_store::StoreExt;

/// Save a filter to use as default
#[tracing::instrument(skip(app_handle))]
#[tauri::command]
pub(crate) async fn set_default_filter(
    app_handle: tauri::AppHandle,
    filter: Filter,
) -> Result<(), Error> {
    if let Some(store) = app_handle.get_store(FILTERS_STORE) {
        store.set(DEFAULT_FILTER_KEY, json!(filter));
        store.save().map_err(|e| Error::StoreError(e.to_string()))?;
        Ok(())
    } else {
        Err(Error::StoreError("Store not loaded".to_string()))
    }
}

#[tracing::instrument(skip(app_handle))]
#[tauri::command]
pub(crate) async fn get_default_filter(app_handle: tauri::AppHandle) -> Result<Filter, Error> {
    if let Some(store) = app_handle.get_store(FILTERS_STORE) {
        if let Some(default_filter) = store.get(DEFAULT_FILTER_KEY) {
            let res: Filter = serde_json::from_value(default_filter)?;
            Ok(res)
        } else {
            Ok(Filter::default())
        }
    } else {
        Err(Error::StoreError("Store not loaded".to_string()))
    }
}

#[tracing::instrument(skip(app_handle))]
#[tauri::command]
pub(crate) async fn save_filters(
    app_handle: tauri::AppHandle,
    filters: HashMap<String, Filter>,
) -> Result<(), Error> {
    // Save the new filters to disk
    if let Some(store) = app_handle.get_store(FILTERS_STORE) {
        store.set(FILTERS_KEY, json!(filters));
        store.save().map_err(|e| Error::StoreError(e.to_string()))?;
        Ok(())
    } else {
        Err(Error::StoreError("Store not loaded".to_string()))
    }
}

#[tracing::instrument(skip(app_handle))]
#[tauri::command]
pub(crate) async fn get_stored_filters(
    app_handle: tauri::AppHandle,
) -> Result<HashMap<String, Filter>, Error> {
    if let Some(store) = app_handle.get_store(FILTERS_STORE) {
        if let Some(store_json) = store.get(FILTERS_KEY) {
            let res: HashMap<String, Filter> = serde_json::from_value(store_json)?;
            Ok(res)
        } else {
            Ok(Filter::default_filters())
        }
    } else {
        Err(Error::StoreError("Store not loaded".to_string()))
    }
}
