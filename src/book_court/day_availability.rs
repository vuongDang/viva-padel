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
                <Space align=SpaceAlign::Center>
                    <p style="font-size: large; font-style: italic; ">{time}</p>
                    {sl
                        .available_courts
                        .into_iter()
                        .map(|court| {
                            view! { <Button style="margin:2px">{court.name.to_string()}</Button> }
                        })
                        .collect_view()}
                </Space>
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
                    let slots = planning.get().1.slots;
                    if slots.is_empty() {
                        view! { <Text>"No available courts"</Text> }
                    } else {
                        slots
                            .into_iter()
                            .map(|(t, s)| {

                                view! { <DayAvailaibilityItem time=t sl=s /> }
                            })
                            .collect_view()
                    }
                }}
            </tbody>
        </Table>
    }
}
