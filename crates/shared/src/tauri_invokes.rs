use crate::frontend::calendar_ui::{DayPlanning, Filter};
use crate::errors::Error;
use leptos::logging::log;
use serde_wasm_bindgen::{from_value, to_value};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

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
struct SaveFilterArgs { 
    filters: HashMap<String, Filter>
}

#[derive(Serialize, Deserialize)]
struct GetFilterArgs { }




impl DayPlanning {
    /// Retrieve the padel courts availaibility planning from the server for the specified date
    #[tracing::instrument]
    pub async fn retrieve(date: &str) -> Result<DayPlanning, Error> {
        let args = to_value(&PlanningArgs { date })?;
        let planning: DayPlanning = from_value(invoke("get_date_planning", args).await)?;
        Ok(planning)
    }
}


impl Filter {
    /// Save filters to the disk.
    #[tracing::instrument]
    pub async fn save_filters(filters: HashMap<String, Filter>) -> Result<(), Error> {
        let args = to_value(&SaveFilterArgs { filters })?;
        let res: JsValue = invoke("save_filters", args).await;
        from_value(res).map_err(|e| Error::WasmConversionError(e.to_string()))
    }

    /// Return filters that were saved on disk.
    /// If no filters were saved we return a default filter.
    #[tracing::instrument]
    pub async fn get_stored_filters() -> Result<HashMap<String, Filter>, Error> {
        let args = to_value(&GetFilterArgs {})?;
        let filters_json = invoke("get_stored_filters", args).await;
        log!("Stored filters: {:?}", filters_json);
        let filters: HashMap<String, Filter> = from_value(filters_json)?;
        log!("Stored filters: {:?}", filters.keys());
        if filters.is_empty() {
            Ok(Filter::default_filters())
        } else {
            Ok(filters)
        }
    }
}

