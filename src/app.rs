use std::thread::spawn;

use crate::{availaibility_calendar::AvailaibilityCalendar, day_availability};
use crate::day_availability::DayAvailaibilityList;
use shared::app_structs::{BookingDuration, DayPlanning};
use serde_wasm_bindgen::{from_value, to_value};
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

#[derive(Serialize, Deserialize)]
struct Empty {

}


#[component]
// let update_name = move |ev| {
pub fn App() -> impl IntoView {
    // let (name, set_name) = create_signal(String::new());
    // let (greet_msg, set_greet_msg) = create_signal(String::new());
    //
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
    
    // let (planning, set_planning) = create_signal(DayPlanning::default());
    // let args = to_value(&Empty{}).unwrap();
    // let task = spawn_local( async move {
    //     let p: DayPlanning = from_value(
    //     invoke("get_local_planning", args).await).unwrap();
    //     set_planning.update(|planning| *planning = p );
    // });
    
                                                        //     spawn_local(async move {
                                                        //     let selected_day = day.date_naive().to_string();
                                                        //     let args = to_value(&PlanningArgs { day: &selected_day })
                                                        //         .unwrap();
                                                        //     let planning: DayPlanningResponse = from_value(
                                                        //             invoke("get_planning", args).await,
                                                        //         )
                                                        //         .expect("Failed to get parse calendar response");
                                                        //     set_msg.update(|msg| *msg = format!("{:#?}", planning));
                                                        // })

    // let planning: DayPlanning = spawn_local(async move {
    //     from_value(
    //     invoke("get_local_planning", args))
    // }).await.expect("Failed to get parse calendar response");

    view! {
        <main class="container">
            <AvailaibilityCalendar />
            // <DayAvailaibilityList day=chrono::Local::now() planning=get_planning() />
        </main>
    }
}
