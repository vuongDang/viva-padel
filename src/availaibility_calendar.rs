use leptos::*;

const DAYS_PER_WEEK: u8 = 7;
const NB_WEEKS_SHOWN: u8 = 4;
const NB_DAYS_SHOWN: u8 = DAYS_PER_WEEK * NB_WEEKS_SHOWN;
use chrono::{DateTime, Datelike, Days, Local, Weekday};

#[component]
pub fn AvailaibilityCalendar() -> impl IntoView {
    let now_datetime = chrono::Local::now();
    let now_day = now_datetime.weekday();
    let days_since_previous_monday = now_day.days_since(Weekday::Mon);

    // The first day we want to show is always a Monday
    let first_day_shown = now_datetime
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
    let days: Vec<Vec<DateTime<Local>>> = days_shown
        .chunks(DAYS_PER_WEEK as usize)
        .map(|s| s.into())
        .collect();

    let (msg, set_msg) = create_signal(String::from("toto"));

    view! {
        {move || msg}
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
                {
                    days.into_iter().map(|week| {
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
                                                    set_msg.update(|msg| *msg = day.day().to_string())
                                                    }
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
            </div>
        </div>
    }
}
