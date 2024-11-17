// use crate::invoke;
use chrono::{DateTime, Local};
use leptos::*;
use leptos_router::*;
// use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::{from_value, to_value};
use shared::frontend::calendar_ui::{DayPlanning, Slot};
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

#[component]
pub(crate) fn DayAvailaibilityList(
    planning: ReadSignal<(Option<String>, DayPlanning)>,
) -> impl IntoView {
    view! {
        <Table>
            <thead>
                <tr>
                    <th>
                        <Text>
                            {move || {
                                format!(
                                    "{} {}",
                                    planning.get().1.weekday,
                                    planning.get().0.unwrap_or_default(),
                                )
                            }}
                        </Text>
                    </th>
                </tr>
            </thead>
            <tbody>
                {move || {
                    planning
                        .get()
                        .1
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
