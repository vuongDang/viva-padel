use crate::book_court::day_availability::DayAvailaibilityList;
use chrono::{DateTime, Datelike, Days, Local, Weekday};
use leptos::*;
use shared::app_structs::DayPlanning;
use shared::DAY_FORMAT;
use thaw::*;
use std::collections::BTreeMap;

const DAYS_PER_WEEK: u8 = 7;
const NB_WEEKS_SHOWN: u8 = 4;
const NB_DAYS_SHOWN: u8 = DAYS_PER_WEEK * NB_WEEKS_SHOWN;

#[component]
pub fn AvailaibilityCalendar() -> impl IntoView {
    // let (days_loaded, set_days_loaded) = create_signal(vec![]);
    // Days shown by the UI
    let (days_shown, set_days_shown) = create_signal::<Vec<Vec<DateTime<Local>>>>(vec![]);
    let show = create_rw_signal(false);
    // Day selected by the user
    let (selected_day, set_selected_day) = create_signal(chrono::Local::now().to_string());

    // Court availabilities for all the days shown
    let calendar = create_rw_signal(BTreeMap::new());

    // Planning for selected day
    let (planning, set_planning) = create_signal(DayPlanning::default());

    // Init by retrieving calendar from server
    let now_datetime = chrono::Local::now();
    set_days_shown.update(|days_shown| *days_shown = get_next_days_from(now_datetime));

    // Closure to update the calendar
    let update_calendar = move  || {spawn_local(async move {
        logging::log!("Updating the calendar");
        let flatten_days = flatten_days(days_shown.get_untracked());
        calendar.update(|cal| {
            for day_shown  in flatten_days.into_iter() {
                cal.entry(day_shown.clone()).or_insert(create_resource(|| (),
                    move |_| {
                        let day_shown_clone = day_shown.clone();
                        async move { 
                            DayPlanning::retrieve(day_shown_clone).await 
                        }}));
            }});
    })};
    update_calendar();


    view! {
        <div id="availability-calendar-prev-next-wrapper">

        <div id="availability-calendar-prev">
            <Button
                // Show previous days
                on:click=move |_| {
                    set_days_shown
                        .update(|days_shown| {
                            let first_day_shown = days_shown[0][0];
                            let next_first_day_shown = first_day_shown
                                .checked_sub_days(Days::new(NB_DAYS_SHOWN as u64))
                                .unwrap();
                            *days_shown = get_next_days_from(next_first_day_shown);
                        });
                    update_calendar();
                    leptos::logging::log!("Prev!");
                }
        color=ButtonColor::Warning >
                Prev
            </Button>
            </div>
            <div
                id="availability-calendar-next"
            >
            <Button
                // Show next days
                on:click=move |_| {
                    set_days_shown
                        .update(|days_shown| {
                            let first_day_shown = days_shown[0][0];
                            let next_first_day_shown = first_day_shown
                                .checked_add_days(Days::new(NB_DAYS_SHOWN as u64))
                                .unwrap();
                            *days_shown = get_next_days_from(next_first_day_shown);
                        });
                    update_calendar();
                    leptos::logging::log!("Next!");
                }
                color=ButtonColor::Warning >
                Next
            </Button>
            </div>
        </div>
        <Table>
            <div id="availability-calendar-headers">
                {(0..DAYS_PER_WEEK)
                    .map(|weekday| {
                        view! {
                            <div class="availability-calendar-headers-cell">
                                {Weekday::try_from(weekday).unwrap().to_string()}
                            </div>
                        }
                    })
                    .collect_view()}

            </div>
            <div id="availability-calendar-body">
                {move || {
                    days_shown
                        .get()
                        .into_iter()
                        .map(|week| {
                            view! {
                                <div class="calendar-week-row">
                                    {week
                                        .into_iter()
                                        .map(|day| {
                                            view! {
                                                <div
                                                    class="calendar-day-cell"
                                                    class=("calendar-day-cell-past", move || now_datetime > day)
                                                    class=(
                                                        "calendar-day-cell-today",
                                                        move || now_datetime == day,
                                                    )
                                                >
                                                    <Transition fallback=move || {
                                                        view! { <Spinner size=SpinnerSize::Medium /> }
                                                    }>
                                                        <Button
                                                            disabled=Signal::derive(move || {
                                                                let day_string = day.format(DAY_FORMAT).to_string();
                                                                logging::log!(
                                                                    "Button {} disabled rendering being processed", day_string
                                                                );
                                                                let calendar = calendar.get();
                                                                let day_planning = calendar.get(&day_string);
                                                                day_planning.is_none() || day_planning.unwrap().get().is_none() || day_planning.unwrap().get().unwrap().day < chrono::Local::now().format(DAY_FORMAT).to_string()
                                                                
                                                            })
                                                            color=ButtonColor::Primary
                                                            on_click=move |_| {
                                                                set_selected_day.set(day.format(DAY_FORMAT).to_string());
                                                                set_planning
                                                                    .set({
                                                                        let calendar = calendar.get();
                                                                        let day_planning = calendar
                                                                            .get(&selected_day.get())
                                                                            .expect("Selected day not in the calendar");
                                                                        day_planning
                                                                            .get()
                                                                            .expect("Day planning ressource should have been loaded")
                                                                            .clone()
                                                                    });
                                                                show.set(true);
                                                            }
                                                        >
                                                            {format!("{}", day.format("%d - %b"))}
                                                        </Button>
                                                    </Transition>
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </Table>
        <Modal show>
            <DayAvailaibilityList planning=planning />
        </Modal>
    }
}

fn get_next_days_from(first_day: DateTime<Local>) -> Vec<Vec<DateTime<Local>>> {
    let now_day = first_day.weekday();
    let days_since_previous_monday = now_day.days_since(Weekday::Mon);

    // The first day we want to show is always a Monday
    let first_day_shown = first_day
        .checked_sub_days(Days::new(days_since_previous_monday as u64))
        .expect("Calendar day underflow");

    // We get all the `NB_DAYS_SHOWN` starting previous Monday
    let days_shown: Vec<DateTime<Local>> = (0..NB_DAYS_SHOWN)
        .map(|i| {
            first_day_shown
                .checked_add_days(Days::new(i as u64))
                .expect("Calendar day overflows")
        })
        .collect();

    // Split the days shown into days per week
    days_shown
        .chunks(DAYS_PER_WEEK as usize)
        .map(|s| s.into())
        .collect()
}

fn flatten_days(days: Vec<Vec<DateTime<Local>>>) -> Vec<String> {
    days.iter()
        .flatten()
        .map(|day_shown| day_shown.format(DAY_FORMAT).to_string())
        .collect()
}
