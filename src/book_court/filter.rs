use chrono::{NaiveTime, Weekday};
use leptos::*;
use shared::{frontend::calendar_ui::Filter, CLOSING_TIME, OPENING_TIME, TIME_FORMAT};
use std::collections::HashSet;
use thaw::*;

const DAYS_PER_WEEK: u8 = 7;

#[component]
pub fn FilterView(filter: RwSignal<Filter>) -> impl IntoView {
    // Days on which the user wants to play
    let weekdays: RwSignal<HashSet<String>> = create_rw_signal(
        filter
            .get_untracked()
            .days_of_the_week
            .into_iter()
            .collect(),
    );

    // Time slots in which the user wants to play
    type TimeSlotsType = RwSignal<Vec<(RwSignal<Option<NaiveTime>>, RwSignal<Option<NaiveTime>>)>>;
    let time_slots: TimeSlotsType = create_rw_signal(
        filter
            .get_untracked()
            .start_time_slots
            .into_iter()
            .map(|(begin, end)| {
                (
                    create_rw_signal(NaiveTime::parse_from_str(&begin, TIME_FORMAT).ok()),
                    create_rw_signal(NaiveTime::parse_from_str(&end, TIME_FORMAT).ok()),
                )
            })
            .collect(),
    );

    // If the user also wants to include outdoor courts
    let with_outdoor = create_rw_signal(filter.get_untracked().with_outdoor);

    // The filter that results from the UI inputs
    let update_filter = Signal::derive(move || {
        let name = String::default();
        let days_of_the_week = weekdays.get().into_iter().collect();
        let start_time_slots = time_slots
            .get()
            .into_iter()
            .map(|(start, end)| {
                (
                    start.get().unwrap().to_string(),
                    end.get().unwrap().to_string(),
                )
            })
            .collect();
        let with_outdoor = with_outdoor.get();
        filter.set(Filter {
            name,
            days_of_the_week,
            start_time_slots,
            with_outdoor,
        })
    });
    let header_style = "background-color: #0078ffaa; padding: 20px;";
    let content_style = "background-color: #0078ff88; padding: 20px;";

    view! {
        <span hidden>{move || format!("{:?}", update_filter.get())}</span>
        <Layout>
            <LayoutHeader style=header_style>"Days"</LayoutHeader>
            <Layout style=content_style>
                <Space align=SpaceAlign::Center>
                    <CheckboxGroup value=weekdays>
                        {get_weekdays_ordered()
                            .iter()
                            .map(|weekday| {
                                view! { <CheckboxItem label=weekday.clone() key=weekday /> }
                            })
                            .collect_view()}
                    </CheckboxGroup>
                    <Button on_click=move |_| { weekdays.set(get_weekdays()) }>"Check all"</Button>
                    <Button on_click=move |_| {
                        weekdays.set(HashSet::new())
                    }>"Uncheck all"</Button>
                </Space>
            </Layout>
            <LayoutHeader style=header_style>"Time range for the start of the game"</LayoutHeader>
            <Layout style=content_style>
                <Space align=SpaceAlign::Center>
                    <Button on_click=move |_| {
                        time_slots.update(|slots| slots.push(default_time_slot()))
                    }>"Add time range"</Button>
                    <Button on_click=move |_| {
                        time_slots
                            .update(|slots| {
                                slots.pop();
                            })
                    }>"Remove time range"</Button>
                </Space>

                {move || {
                    time_slots
                        .get()
                        .into_iter()
                        .map(|slot| {
                            view! {
                                <Space align=SpaceAlign::Center>
                                    "Start" <TimePicker value=slot.0 /> "End"
                                    <TimePicker value=slot.1 />
                                </Space>
                            }
                        })
                        .collect_view()
                }}
            </Layout>
            <LayoutHeader style=header_style>"Outdoor"</LayoutHeader>
            <Layout style=content_style>
                <Checkbox value=with_outdoor>"With outdoor courts"</Checkbox>
            </Layout>
        </Layout>
        <div style="margin-top: 1rem">
            <p>"filter: " {move || format!("{:#?}", filter.get())}</p>
        </div>
    }
}

fn get_weekdays() -> HashSet<String> {
    (0..DAYS_PER_WEEK)
        .map(|weekday| Weekday::try_from(weekday).unwrap().to_string())
        .collect()
}

fn get_weekdays_ordered() -> Vec<String> {
    (0..DAYS_PER_WEEK)
        .map(|weekday| Weekday::try_from(weekday).unwrap().to_string())
        .collect()
}

fn default_time_slot() -> (RwSignal<Option<NaiveTime>>, RwSignal<Option<NaiveTime>>) {
    (
        create_rw_signal(Some(
            NaiveTime::parse_from_str(OPENING_TIME, TIME_FORMAT).unwrap(),
        )),
        create_rw_signal(Some(
            NaiveTime::parse_from_str(CLOSING_TIME, TIME_FORMAT).unwrap(),
        )),
    )
}
