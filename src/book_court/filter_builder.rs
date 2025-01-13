use chrono::{NaiveTime, Weekday};
use leptos::{spawn_local, *};
use shared::{frontend::calendar_ui::Filter, CLOSING_TIME, OPENING_TIME, TIME_FORMAT};
use std::collections::{HashMap, HashSet};
use thaw::*;
use tracing::*;

const DAYS_PER_WEEK: u8 = 7;
type TimeSlotsType = RwSignal<Vec<(RwSignal<Option<NaiveTime>>, RwSignal<Option<NaiveTime>>)>>;
const HEADER_STYLE: &str = "background-color: #0078ffaa; padding: 20px;";
const CONTENT_STYLE: &str = "background-color: #0078ff88; padding: 20px;";

#[tracing::instrument]
#[component]
pub fn FilterView(
    filters: RwSignal<Option<HashMap<String, Filter>>>,
) -> impl IntoView {
    let active_filter = use_context::<RwSignal<Option<Filter>>>().expect("Filter not found in context");
    // Are we reading or editing a filter
    let read_mode = create_rw_signal(true);

    // Name of the filter
    let filter_name: RwSignal<String> =
        create_rw_signal(active_filter.get_untracked().unwrap().name);

    // Days on which the user wants to play
    let weekdays: RwSignal<HashSet<String>> = create_rw_signal(
        active_filter
            .get_untracked()
            .unwrap()
            .days_of_the_week
            .into_iter()
            .collect(),
    );

    // Time slots in which the user wants to play
    let time_slots: TimeSlotsType = create_rw_signal(
        active_filter
            .get_untracked()
            .unwrap()
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
    let with_outdoor = create_rw_signal(active_filter.get_untracked().unwrap().with_outdoor);

    // Save the filter that results from the UI inputs
    let save_filter = move || {
        let name = filter_name.get_untracked();
        let name_clone = name.clone();
        let days_of_the_week = weekdays.get_untracked().into_iter().collect();
        let start_time_slots = time_slots
            .get_untracked()
            .into_iter()
            .map(|(start, end)| {
                (
                    start.get_untracked().unwrap().to_string(),
                    end.get_untracked().unwrap().to_string(),
                )
            })
            .collect();
        let with_outdoor = with_outdoor.get();
        let filter = Filter {
            name,
            days_of_the_week,
            start_time_slots,
            with_outdoor,
        };

        trace!("Saving filter: {:?}", filter);
        // Save it in the stored filters
        filters.update(|filters| {
            if let Some(filters) = filters {
                let _ = filters.insert(name_clone, filter.clone());
            }
        });
        spawn_local(async move {
            if let Some(filters) = filters.get() {
                Filter::save_filters(filters).await.expect("Failed to save filters to the disk store")
            }
        });

        // Set it as the active filter
        active_filter.set(Some(filter));
    };

    // Remove a filter
    let remove_filter = move || {
        let name = active_filter.get_untracked().unwrap().name;

        trace!("Removing filter: {:?}", name);
        // Save it in the stored filters
        filters.update(|filters| {
            if let Some(filters) = filters {
                let _ = filters.remove(&name);
            }
        });

        spawn_local(async move {
            if let Some(filters) = filters.get() {
                Filter::save_filters(filters).await.expect("Failed to save filters to the disk store")
            }
        });

        if let Some(filters) = filters.get() {
            active_filter.set(Some(filters.iter().next().unwrap().1.clone()));
        }
    };

    view! {
        <Layout>
            <Layout style=HEADER_STYLE>
                <Space align=SpaceAlign::Center>
                    <Text>"Filter Builder"</Text>
                    <Button
                        disabled=Signal::derive(move || !read_mode.get())
                        on_click=move |_| { read_mode.set(false) }
                    >
                        "Create/Modify"
                    </Button>
                    <Button
                        disabled=read_mode
                        on_click=move |_| {
                            read_mode.set(true);
                            save_filter();
                        }
                    >
                        "Save"
                    </Button>
                    <Button disabled=read_mode on_click=move |_| { read_mode.set(true) }>
                        "Cancel"
                    </Button>
                    <Button
                        disabled=Signal::derive(move || {
                            filters.get().is_none() || filters.get().unwrap().len() <= 1
                                || !read_mode.get()
                        })
                        on_click=move |_| {
                            read_mode.set(true);
                            remove_filter();
                        }
                    >
                        "Remove"
                    </Button>
                </Space>
            </Layout>
            <Show
                when=move || read_mode.get()
                fallback=move || {
                    view! { <FilterEditView filter_name weekdays time_slots with_outdoor /> }
                }
            >
                <FilterReadView active_filter />
            </Show>
        </Layout>
    }
}

#[component]
fn FilterReadView(active_filter: RwSignal<Option<Filter>>) -> impl IntoView {
    view! {
        <Layout style=CONTENT_STYLE>
            {move || {
                let filter = active_filter.get().unwrap();
                view! {
                    <Space vertical=true>
                        <Text>"Filter name: " {format!("{:?}", filter.name)}</Text>
                        <Text>"Weekdays: " {format!("{:?}", filter.days_of_the_week)}</Text>
                        <Text>
                            "Game starts between: " {format!("{:?}", filter.start_time_slots)}
                        </Text>
                        <Text>
                            "Include outdoor courts: "{format!("{:?}", filter.with_outdoor)}
                        </Text>
                    </Space>
                }
            }}
        </Layout>
    }
}

#[component]
fn FilterEditView(
    filter_name: RwSignal<String>,
    weekdays: RwSignal<HashSet<String>>,
    time_slots: TimeSlotsType,
    with_outdoor: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Layout style=CONTENT_STYLE>
            <Space align=SpaceAlign::Center>
                <Text>"Filter Name"</Text>
                <Input value=filter_name />
            </Space>
        </Layout>
        <Layout style=CONTENT_STYLE>
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
                <Button on_click=move |_| { weekdays.set(HashSet::new()) }>"Uncheck all"</Button>
            </Space>
        </Layout>
        <Layout style=CONTENT_STYLE>
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
            <Space align=SpaceAlign::Center>
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
            </Space>
        </Layout>
        <Layout style=CONTENT_STYLE>
            <Checkbox value=with_outdoor>"With outdoor courts"</Checkbox>
        </Layout>
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
