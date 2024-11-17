use crate::frontend::calendar_ui::DayPlanning;
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
    date: &'a str,
}

impl DayPlanning {
   pub async fn retrieve(date: &str) -> DayPlanning {
        let args = to_value(&PlanningArgs { date }).unwrap();
        let planning: DayPlanning = from_value(invoke("get_date_planning", args).await)
            .expect("Failed to get parse calendar response");
        // leptos::logging::log!("[DayPlanning::retrieve] End for day: {}", planning.weekday);
        planning
    }
}


