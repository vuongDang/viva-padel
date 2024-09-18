use crate::invoke;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use shared_structs::server_structs::PlanningResponse;

const DAYS_PER_WEEK: u8 = 7;
const NB_WEEKS_SHOWN: u8 = 4;
const NB_DAYS_SHOWN: u8 = DAYS_PER_WEEK * NB_WEEKS_SHOWN;
use chrono::{DateTime, Datelike, Days, Local, Weekday};

#[derive(Serialize, Deserialize)]
struct PlanningArgs<'a> {
    day: &'a str,
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

#[component]
pub fn AvailaibilityCalendar() -> impl IntoView {
    let (msg, set_msg) = create_signal(String::from("toto"));
    let (days_shown, set_days_shown) = create_signal::<Vec<Vec<DateTime<Local>>>>(vec![]);

    let now_datetime = chrono::Local::now();

    let days = get_next_days_from(now_datetime);
    set_days_shown.update(|days_shown| *days_shown = days);

    view! {
        {move || msg}
        <div id="availability-calendar-prev-next-wrapper">
        <div id="availability-calendar-prev"
            // Show previous days
            on:click={ move |_| {
                set_days_shown.update(|days_shown| {
                    let first_day_shown = days_shown[0][0];
                    let next_first_day_shown = first_day_shown.checked_sub_days(Days::new(NB_DAYS_SHOWN as u64)).unwrap();
                    *days_shown = get_next_days_from(next_first_day_shown);
                })
            }} >
           Prev
        </div>
        <div id="availability-calendar-next"
            // Show next days
            on:click={ move |_| {
                set_days_shown.update(|days_shown| {
                    let first_day_shown = days_shown[0][0];
                    let next_first_day_shown = first_day_shown.checked_add_days(Days::new(NB_DAYS_SHOWN as u64)).unwrap();
                    *days_shown = get_next_days_from(next_first_day_shown);
                })
            }}>
            Next
        </div>
        </div>
        <div id="availability-calendar">
            <div id="availability-calendar-headers">
                {
                    (0..DAYS_PER_WEEK).map(|weekday| {
                        view! {
                            <div class="availability-calendar-headers-cell">
                                {Weekday::try_from(weekday).unwrap().to_string()}
                            </div>
                        }
                    }).collect_view()

                }
            </div>
            <div id="availability-calendar-body">
                {move || {
                        days_shown.get().into_iter().map(|week| {
                            view! {
                                <div class="calendar-week-row">
                                    {
                                        week.into_iter().map(|day| {
                                            view! {
                                                // <a href="#" class="calendar-day-cell">
                                                <div
                                                    class="calendar-day-cell"
                                                    class=("calendar-day-cell-past", move || now_datetime > day)
                                                    class=("calendar-day-cell-today", move || now_datetime.day() == day.day())
                                                    on:click={ move |_| {
                                                        spawn_local(async move {
                                                            let selected_day = day.date_naive().to_string();
                                                            let args = to_value(&PlanningArgs { day: &selected_day }).unwrap();
                                                            let planning: PlanningResponse = from_value(invoke("get_planning", args).await).expect("Failed to get parse calendar response");
                                                            set_msg.update(|msg| *msg = format!("{:#?}", planning));
                                                        }
                                                        )}
                                                    }
                                                    >
                                                    {format!("{}", day.format("%d - %b"))}
                                                </div>
                                                // </a>
                                            }
                                        }).collect_view()
                                    }
                                </div>
                            }
                        }).collect_view()
                    }
                }
            </div>
        </div>
    }
}
