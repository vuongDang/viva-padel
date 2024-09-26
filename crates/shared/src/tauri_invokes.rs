use crate::app_structs::DayPlanning;
use serde_wasm_bindgen::{from_value, to_value};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub(crate) async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct PlanningArgs<'a> {
    day: &'a str,
}

impl DayPlanning {
   pub async fn retrieve(day: String) -> DayPlanning {
        leptos::logging::log!("[DayPlanning::retrieve] Start for day: {}", day);
        let args = to_value(&PlanningArgs { day: &day }).unwrap();
        let planning: DayPlanning = from_value(invoke("get_day_planning", args).await)
            .expect("Failed to get parse calendar response");
        leptos::logging::log!("[DayPlanning::retrieve] End for day: {}", day);
        planning
    }
}


