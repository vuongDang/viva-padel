use crate::availaibility_calendar::AvailaibilityCalendar;
use crate::day_availability::DayAvailaibilityList;
// use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub(crate) async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub(crate) fn get_planning() -> shared::app_structs::DayPlanning {
    let manifest_path = std::env!("CARGO_MANIFEST_DIR");
    let path = "tests/json_responses/get_planning.json";
    let path = format!("{manifest_path}/{path}");
    let response = std::fs::read_to_string(path).expect("Failed to read json file");
    let parsed = serde_json::from_str::<shared::server_structs::DayPlanningResponse>(&response);
    parsed.unwrap().into()
}

#[component]
pub fn App() -> impl IntoView {
    // let (name, set_name) = create_signal(String::new());
    // let (greet_msg, set_greet_msg) = create_signal(String::new());
    //
    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };
    //
    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         let name = name.get_untracked();
    //         if name.is_empty() {
    //             return;
    //         }
    //
    //         // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //         let args = to_value(&GreetArgs { name: &name }).unwrap();
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };

    view! {
        <main class="container">
            // <AvailaibilityCalendar />
            <DayAvailaibilityList day=chrono::Local::now() planning=get_planning() />
        </main>
    }
}
