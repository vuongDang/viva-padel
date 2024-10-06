// use crate::invoke;
use chrono::{DateTime, Local};
use leptos::*;
use leptos_router::*;
// use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::{from_value, to_value};
use shared::app_structs::{DayPlanning, Slot};
use thaw::*;
// use shared::server_structs::DayPlanningResponse;

#[derive(Params, PartialEq)]
struct UIDay {
    day: Option<DateTime<Local>>,
}

#[component]
pub fn DayAvailaibilityItem(time: String, sl: Slot) -> impl IntoView {
    view! {
        <tr>
            <td>
                <p class="day-avaibility-item-time">{time}</p>
                <p>
                    {sl
                        .available_courts
                        .iter()
                        .map(|court| court.name.to_string())
                        .collect::<Vec<String>>()
                        .join(" - ")}
                </p>
            </td>
        </tr>
    }
}

// #[component]
// pub fn DayAvailaibilityList(planning: DayPlanning) -> impl IntoView {
//     let params = leptos_router::use_params::<UIDay>();
//     let day = move || params.with(|params| params.as_ref().map(|p| p.day.unwrap()).unwrap());
//     view! {
//         <Table>
//             <thead>
//                 <tr>
//                     <th>
//                         <Text>{day().date_naive().format("%A %d %B %C%y").to_string()}</Text>
//                     </th>
//                 </tr>
//             </thead>
//             <tbody>
//                 {planning
//                     .slots
//                     .into_iter()
//                     .map(|(t, s)| {
//                         view! { <DayAvailaibilityItem time=t sl=s /> }
//                     })
//                     .collect_view()}
//             </tbody>
//         </Table>
//     }
// }

#[component]
pub(crate) fn DayAvailaibilityList(planning: ReadSignal<DayPlanning>) -> impl IntoView {
    view! {
        <Table>
            <thead>
                <tr>
                    <th>
                        <Text>{move || planning.get().day}</Text>
                    </th>
                </tr>
            </thead>
            <tbody>
                {move || {
                    planning
                        .get()
                        .slots
                        .into_iter()
                        .map(|(t, s)| {
                            view! { <DayAvailaibilityItem time=t sl=s /> }
                        })
                        .collect_view()
                }}
            </tbody>
        </Table>
    }
}
