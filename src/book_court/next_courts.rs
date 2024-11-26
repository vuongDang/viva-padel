use crate::book_court::day_availability::DayAvailaibilityItem;
use leptos::*;
use shared::frontend::calendar_ui::{Calendar, DateKey, DayPlanning, Slot, StartTime};
use shared::DATE_FORMAT;
use std::collections::BTreeMap;
use thaw::*;
use tracing::*;

const NB_ITEMS_AT_START: usize = 3;

#[component]
pub fn NextCourtsView(
    calendar: RwSignal<Calendar>,
    filtered_calendar: Memo<BTreeMap<DateKey, DayPlanning>>,
) -> impl IntoView {
    let nb_items = create_rw_signal(NB_ITEMS_AT_START);
    let next_courts_found = Signal::derive(move || {
        trace!("NextCourtsView: next_courts_found signal");
        let mut next_date_to_poll = chrono::Local::now().date_naive();
        let mut next_courts: Vec<((String, DateKey), StartTime, Slot)> = vec![];
        let mut nb_courts_found = 0;
        while nb_items.get() > nb_courts_found {
            let day_planning = filtered_calendar.get();

            let next_date_string = &next_date_to_poll.format(DATE_FORMAT).to_string();
            let time_slot = day_planning.get(next_date_string);
            if time_slot.is_none() {
                // Calendar have not fetched up to this date yet
                break;
            }
            let slots = &time_slot.unwrap().slots;
            let weekday = &time_slot.unwrap().weekday;
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
        }
        next_courts
    });

    view! {
        <Space align=SpaceAlign::Center>
            <Text>"Number of items shown: "</Text>
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
                                // let slot: Option<(DateKey, StartTime, Slot)> = next_courts
                                <tr>
                                    <td style="padding: 0px">
                                        <Show
                                            when=move || { slot_clone.is_some() }
                                            fallback=|| {
                                                view! { <p>"Not found yet"</p> }
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
                        <Button style="width:100%;">"Load more courts"</Button>
                    </Layout>
                </Layout>

            </tbody>
        </Table>
    }
}
