// use crate::invoke;
use chrono::{DateTime, Datelike, Days, Local, Weekday};
use leptos::*;
// use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::{from_value, to_value};
use shared::app_structs::DayPlanning;
// use shared::server_structs::DayPlanningResponse;

#[component]
pub fn DayAvailaibilityList(day: DateTime<Local>, planning: DayPlanning) -> impl IntoView {
    view! {
        <p>{format!("{:?}", day.date_naive())}</p>
        <div id="day-availability-table">
            <div id="day-availability-body">
                {planning
                    .slots
                    .into_iter()
                    .map(|(time, slot)| {
                        view! {
                            <div id="day-availability-row">
                                <div id="day-availability-cell">
                                    <p>"Time: " {time}</p>
                                    <p>
                                        "Available courts: "
                                        {slot
                                            .available_courts
                                            .iter()
                                            .map(|court| court.name.to_string())
                                            .collect::<Vec<String>>()
                                            .join(" - ")}
                                    </p>
                                </div>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
