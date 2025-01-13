use crate::errors::Error;
use crate::frontend::calendar_ui::{DayPlanning, Filter};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::collections::HashMap;
use tracing::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub(crate) async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct PlanningArgs<'a> {
    date: &'a str,
}

#[derive(Serialize, Deserialize)]
struct SaveFiltersArgs {
    filters: HashMap<String, Filter>,
}

#[derive(Serialize, Deserialize)]
struct NoArgs {}

#[derive(Serialize, Deserialize)]
struct SetDefaultFilterArgs {
    filter: Filter,
}

impl DayPlanning {
    /// Retrieve the padel courts availaibility planning from the server for the specified date
    #[tracing::instrument]
    pub async fn retrieve(date: &str) -> Result<DayPlanning, Error> {
        trace!("DayPlanning::retrieve: {}", date);
        let args = to_value(&PlanningArgs { date })?;
        let planning: DayPlanning = from_value(invoke("get_date_planning", args).await)?;
        Ok(planning)
    }
}

impl Filter {
    /// Save filters to the disk.
    #[tracing::instrument]
    pub async fn save_filters(filters: HashMap<String, Filter>) -> Result<(), Error> {
        trace!("Filter::save_filters: {:?}", filters);
        let args = to_value(&SaveFiltersArgs { filters })?;
        let res: JsValue = invoke("save_filters", args).await;
        from_value(res).map_err(|e| Error::WasmConversionError(e.to_string()))
    }

    /// Return filters that were saved on disk.
    /// If no filters were saved we return a default filter.
    #[tracing::instrument]
    pub async fn get_stored_filters() -> Result<HashMap<String, Filter>, Error> {
        trace!("Filter::get_stored_filters");
        let args = to_value(&NoArgs {})?;
        let filters_json = invoke("get_stored_filters", args).await;
        let filters: HashMap<String, Filter> = from_value(filters_json)?;
        if filters.is_empty() {
            Ok(Filter::default_filters())
        } else {
            Ok(filters)
        }
    }

    /// Save filters to the disk.
    #[tracing::instrument]
    pub async fn set_default_filter(filter: Filter) -> Result<(), Error> {
        trace!("Filter::set_default_filter: {:?}", filter);
        let args = to_value(&SetDefaultFilterArgs { filter })?;
        let res: JsValue = invoke("set_default_filter", args).await;
        from_value(res).map_err(|e| Error::WasmConversionError(e.to_string()))
    }

    /// Return filters that were saved on disk.
    /// If no filters were saved we return a default filter.
    #[tracing::instrument]
    pub async fn get_default_filter() -> Result<Filter, Error> {
        trace!("Filter::get_default_filter");
        let args = to_value(&NoArgs {})?;
        let filter_json = invoke("get_default_filter", args).await;
        let filter: Filter = from_value(filter_json)?;
        Ok(filter)
    }
}
