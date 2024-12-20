use crate::book_court::{
    day_availability::DayAvailaibilityItem, update_calendar, FilteredCalendar,
};
use leptos::*;
use shared::frontend::calendar_ui::{Calendar, DateKey, DayPlanning, Slot, StartTime};
use shared::DATE_FORMAT;
use std::collections::BTreeMap;
use thaw::*;
use tracing::*;
use web_sys::console::trace;

const NB_ITEMS_AT_START: usize = 4;

#[component]
pub fn NextCourtsView(
    calendar: RwSignal<Calendar>,
    filtered_calendar: FilteredCalendar,
) -> impl IntoView {
    let nb_items = create_rw_signal(NB_ITEMS_AT_START);
    let next_courts_found = Signal::derive(move || {
        trace!("NextCourtsView: compute list of [`next_courts_found`]");
        let mut next_date_to_poll = chrono::Local::now().date_naive();
        let mut next_courts: Vec<((String, DateKey), StartTime, Slot)> = vec![];
        let mut nb_courts_found = 0;
        let filtered_calendar = filtered_calendar.get();
        while nb_items.get() > nb_courts_found {
            let next_date_string = next_date_to_poll.format(DATE_FORMAT).to_string();
            let day_planning = filtered_calendar.get(&next_date_string);
            trace!(
                "date: {}, day_planning: {:?}",
                next_date_string,
                day_planning
            );
            if let Some(day_planning_signal) = day_planning {
                // Calendar has started loaded planning for this date
                if let Some(time_slot) = day_planning_signal.get() {
                    trace!(
                        "Date has been loaded: {:?} --- Calendar keys: {:?}",
                        next_date_string,
                        calendar.get()
                    );
                    // Calendar has finished loading planning for this date
                    let slots = &time_slot.slots;
                    let weekday = &time_slot.weekday;
                    for (start, slot) in slots.iter() {
                        if !slot.available_courts.is_empty() {
                            next_courts.push((
                                (weekday.clone(), next_date_string.clone()),
                                start.clone(),
                                slot.clone(),
                            ));
                            nb_courts_found += 1;
                        }
                    }
                    next_date_to_poll = next_date_to_poll
                        .checked_add_days(chrono::Days::new(1))
                        .expect("Reach end of days");
                    continue;
                } else {
                    // The calendar is still loading data from server, just stop the computation
                    // and it will be retriggered later when data has been loaded
                    trace!(
                        "Date still loading: {:?} --- Calendar keys: {:?}",
                        next_date_string,
                        calendar.get()
                    );
                    break;
                }
            } else {
                // The calendar has not tried to load this date yet, request new date to be loaded
                // and stop current computation
                trace!(
                    "No date {:?} in the calendar --- Calendar state: {:?}",
                    next_date_string,
                    calendar.get()
                );
                update_calendar(calendar, vec![next_date_string]);
                break;
            }
        }
        next_courts
    });

    view! {
        <Space align=SpaceAlign::Center>
            <Text>"Number of items loaded per batch: "</Text>
            <InputNumber value=nb_items step=1 />
        </Space>
        <Table style="--thaw-border-color: black; --thaw-background-color: #FDE992">
            <tbody>
                {move || {
                    (0..nb_items.get())
                        .map(|i| {
                            let next_courts = next_courts_found.get();
                            let slot: Option<((String, DateKey), StartTime, Slot)> = next_courts
                                .get(i)
                                .cloned();
                            let slot_clone = slot.clone();
                            view! {
                                <tr>
                                    <td style="padding: 0px">
                                        <Show
                                            when=move || { slot_clone.is_some() }
                                            fallback=|| {
                                                view! { <Spinner size=SpinnerSize::Medium /> }
                                            }
                                        >
                                            {
                                                let slot = slot.clone().unwrap();
                                                let weekday = slot.0.0;
                                                let date = slot.0.1;
                                                let time = slot.1;
                                                let available_courts = slot.2;
                                                view! {
                                                    <Layout>
                                                        <LayoutHeader style="background-color: #E3B778; padding: 0.5em; font-size: large; font-weight: bold;">
                                                            {weekday} " "{date}
                                                        </LayoutHeader>
                                                        <Layout>
                                                            <DayAvailaibilityItem time sl=available_courts />
                                                        </Layout>
                                                    </Layout>
                                                }
                                            }
                                        </Show>
                                    </td>
                                </tr>
                            }
                        })
                        .collect_view()
                }} <Layout>
                    <Layout>
                        <Button
                            style="width:100%;"
                            on_click=move |_| {
                                nb_items.update(|nb| *nb = *nb + NB_ITEMS_AT_START)
                            }
                        >
                            "Load more courts"
                        </Button>
                    </Layout>
                </Layout>

            </tbody>
        </Table>
    }
}
