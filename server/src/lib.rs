mod alarm_logic;
pub mod api;
pub mod auth;
#[cfg(feature = "local_dev")]
pub mod mock;
pub mod models;
pub mod services;

use crate::{
    models::legarden::{Availabilities, Court, DayPlanningResponse, Slot},
    services::legarden::{DATE_FORMAT, TIME_FORMAT},
};
use chrono::{Datelike, Month, Timelike, Weekday};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::time::sleep;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use crate::models::Alarm;
use crate::services::{DataBaseService, LeGardenService, NotificationsService};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Calendar {
    pub timestamp: i64,
    pub availabilities: Availabilities,
}

#[derive(Clone)]
pub struct AppState {
    /// A calendar containing LeGarden availabilities
    pub calendar: Arc<RwLock<Calendar>>,
    /// Database service
    pub db: Arc<dyn DataBaseService>,
    /// LeGarden service
    pub legarden: Arc<dyn LeGardenService>,
    /// Notifications service
    pub notifications: Arc<dyn NotificationsService>,
    /// Secret key for JWT signing/verification
    pub jwt_secret: String,
}

pub async fn run(state: AppState) {
    const MAX_RETRIES: u32 = 3;
    const RETRY_DELAY_SECS: u64 = 5;
    loop {
        let time_now = chrono::Local::now().hour();
        if state.legarden.polling_time().contains(&time_now)
            || state
                .calendar
                .read()
                .expect("Failed to get calendar")
                .availabilities
                .is_empty()
        {
            tracing::info!("Polling calendar");
            for attempt in 1..=MAX_RETRIES {
                match state.legarden.get_calendar().await {
                    Ok(availabilities) => {
                        let timestamp = chrono::Local::now().timestamp();
                        let mut cal = state.calendar.write().expect("Failed to get mut guard");

                        let new = availabilities.clone();
                        let old = cal.availabilities.clone();
                        let state_clone = state.clone();
                        tokio::spawn(async move {
                            notify_users_for_freed_courts(state_clone, new, old).await
                        });

                        cal.availabilities = availabilities;
                        cal.timestamp = timestamp;
                        tracing::info!("Calendar fetched!");
                        break;
                    }
                    Err(e) => {
                        if attempt < MAX_RETRIES {
                            tracing::warn!("Failed to get calendar: {e}. Retrying...");
                            sleep(Duration::from_secs(RETRY_DELAY_SECS)).await;
                        } else {
                            tracing::error!(
                                "Could not get calendar after {MAX_RETRIES} attempts. Giving up"
                            );
                            // Decide what to do if we can't fetch the calendar
                            // For now just continue
                            break;
                        }
                    }
                }
            }
        } else {
            tracing::info!("Not the time to poll, let's sleep... zzzZZZzzZZZ");
        }

        let sleep_duration = state.legarden.polling_interval();
        tracing::info!("Waiting for {:?} before next poll.", sleep_duration);
        sleep(sleep_duration).await;
    }
}

pub async fn notify_users_for_freed_courts(
    state: AppState,
    new: Availabilities,
    old: Availabilities,
) -> HashMap<Uuid, HashMap<String, Availabilities>> {
    let freed_courts = freed_courts(&new, &old);

    if freed_courts.is_empty() {
        return HashMap::new();
    }

    let alarms_data = state.db.get_active_alarms().await.unwrap_or_else(|e| {
        tracing::error!("Failed to fetch alarms from the db: {}", e);
        vec![]
    });

    // Group alarms by user
    let mut alarms_by_user: HashMap<Uuid, Vec<Alarm>> = HashMap::new();
    for (user_id, alarm) in alarms_data {
        alarms_by_user.entry(user_id).or_default().push(alarm);
    }

    // For each user and its alarms keep the corresponding availabilities
    let avail_filtered_with_alarms: HashMap<Uuid, HashMap<String, Availabilities>> = alarms_by_user
        .into_iter()
        .map(|(user_id, alarms)| {
            let mut avails = HashMap::new();
            for alarm in alarms {
                avails.insert(
                    alarm.name.clone(),
                    alarm.target_availabilities(freed_courts.clone()),
                );
            }
            (user_id, avails)
        })
        .collect();

    if !avail_filtered_with_alarms.is_empty() {
        send_notifications_to_users(state.clone(), &avail_filtered_with_alarms).await;
    }
    avail_filtered_with_alarms
}

/// Gather courts that got freed between old and new availabilities
/// and are at least `NB_MINUTES_THRESHOLD_FOR_NOTIF` away from current time
const NB_MINUTES_THRESHOLD_FOR_NOTIF: i64 = 60;
pub fn freed_courts(new: &Availabilities, old: &Availabilities) -> Availabilities {
    let mut freed = BTreeMap::new();

    let dates = old.keys().filter(|date| new.contains_key(*date));

    let now = chrono::Local::now().naive_local();
    for day_str in dates {
        if let Some(new_day) = new.get(day_str) {
            let old_day = old.get(day_str).unwrap();
            let mut courts = vec![];
            for (old_court, new_court) in old_day.courts().iter().zip(new_day.courts().iter()) {
                let mut slots = vec![];
                for (old_slot, new_slot) in old_court.slots().iter().zip(new_court.slots().iter()) {
                    let mut prices = vec![];
                    if !is_slot_too_soon_or_already_passed(
                        now,
                        day_str,
                        &old_slot,
                        NB_MINUTES_THRESHOLD_FOR_NOTIF,
                    ) {
                        for (old_price, new_price) in
                            old_slot.prices().iter().zip(new_slot.prices().iter())
                        {
                            if !old_price.bookable() && new_price.bookable() {
                                prices.push(new_price.clone());
                            }
                        }
                    }
                    if !prices.is_empty() {
                        slots.push(Slot::clone_with_prices(new_slot, prices));
                    }
                }
                if !slots.is_empty() {
                    courts.push(Court::clone_with(new_court, slots));
                }
            }
            if !courts.is_empty() {
                freed.insert(
                    day_str.to_owned(),
                    DayPlanningResponse::new_with(new_day, courts),
                );
            }
        }
    }
    Availabilities(freed)
}

pub(crate) async fn send_notifications_to_users(
    state: AppState,
    avail_filtered_with_alarms: &HashMap<Uuid, HashMap<String, Availabilities>>,
) {
    for (user_id, availabilities_per_alarm) in avail_filtered_with_alarms {
        // 1. Get tokens for this user
        let tokens = match state.db.get_tokens_for_user(*user_id).await {
            Ok(tokens) => tokens,
            Err(e) => {
                tracing::error!("Failed to fetch tokens for user {}: {}", user_id, e);
                continue;
            }
        };

        if tokens.is_empty() {
            tracing::info!(
                "No registered devices for user {}, skipping notification",
                user_id
            );
            continue;
        }

        // 2. Build the message
        let alarm_names: Vec<String> = availabilities_per_alarm.keys().cloned().collect();
        let availabilities: Vec<Availabilities> =
            availabilities_per_alarm.values().cloned().collect();

        // If availabilities are all empty, don't send notifications
        if availabilities.iter().all(|avail| avail.iter().count() == 0) {
            continue;
        }

        let title = "Courts libérés! 🎾";
        let body = message_from_availabilities(&availabilities);

        // 3. Send
        let data = Some(serde_json::json!({ "user_id": user_id, "alarms": alarm_names }));
        if let Err(e) = state
            .notifications
            .send_notification(&tokens, title, &body, data)
            .await
        {
            tracing::error!("Failed to send notification to user {}: {}", user_id, e);
        } else {
            tracing::info!("Notification sent successfully to user {}", user_id);
        }
    }
}

fn message_from_availabilities(avail: &Vec<Availabilities>) -> String {
    let mut messages = vec![];
    for avail in avail {
        for (date, _court, slot, _price) in avail.iter() {
            let date = chrono::NaiveDate::parse_from_str(date, DATE_FORMAT)
                .expect("Failed to format date");
            let weekday = weekday_to_french(date.weekday());
            let date_str = date.format("%d/%m").to_string();
            let msg = format!("{} {} à {}", weekday, date_str, slot.start_at());
            messages.push(msg);
        }
    }
    messages.dedup();
    messages.join("\n")
}

fn weekday_to_french(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "Lun",
        Weekday::Tue => "Mar",
        Weekday::Wed => "Mer",
        Weekday::Thu => "Jeu",
        Weekday::Fri => "Ven",
        Weekday::Sat => "Sam",
        Weekday::Sun => "Dim",
    }
}

#[allow(dead_code)]
fn months_to_french(month: Month) -> &'static str {
    match month {
        Month::January => "Jan",
        Month::February => "Fév",
        Month::March => "Mars",
        Month::April => "Avr",
        Month::May => "Mai",
        Month::June => "Juin",
        Month::July => "Juil",
        Month::August => "Août",
        Month::September => "Sept",
        Month::October => "Oct",
        Month::November => "Nov",
        Month::December => "Déc",
    }
}

#[cfg(all(test, feature = "local_dev"))]
mod tests {
    use std::collections::BTreeMap;

    use chrono::{NaiveTime, TimeDelta};
    use testcases::legarden::{json_planning_simple_all_booked, json_planning_simple_day};

    use crate::{
        mock::simple_availabilities_with_start_tomorrow,
        models::{CourtType, legarden::DayPlanningResponse},
    };

    use super::*;

    #[tokio::test]
    async fn test_notify_users_for_freed_courts() {
        let (_, state) = crate::mock::default_test_server().await;
        let email = "toto@toto.com";
        let token = "notif_token";
        let device_id = "device";
        let user = state.db.create_user(email).await.unwrap();
        // Setup device
        state
            .db
            .register_device(device_id, token, user.id)
            .await
            .unwrap();

        // Setup alarm
        let alarm = Alarm {
            name: "Morning Indoor".to_string(),
            court_type: CourtType::Indoor,
            time_range: (
                NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            ),
            is_active: true,
            ..Default::default()
        };
        state.db.update_alarms(user.id, vec![alarm]).await.unwrap();

        // 4. Construct old and new availabilities
        // We want a slot that was NOT bookable in old, but IS bookable in new.
        let old = simple_availabilities_with_start_tomorrow(2, json_planning_simple_all_booked());
        let new = simple_availabilities_with_start_tomorrow(2, json_planning_simple_day());

        // 5. Run the notification logic
        let results = notify_users_for_freed_courts(state, new, old).await;

        // 6. Assertions
        assert!(results.contains_key(&user.id));
        let user_results = results.get(&user.id).unwrap();
        assert!(user_results.contains_key("Morning Indoor"));
        let notified_avail = user_results.get("Morning Indoor").unwrap();

        assert_eq!(notified_avail.iter().count(), 4);
        for (_, court, slot, price) in notified_avail.iter() {
            assert!(price.bookable());
            assert_eq!(slot.start_at(), "10:00");
            assert_eq!(court.name(), "Padel 1")
        }
    }

    #[test]
    fn freed_courts_correct_simple() {
        let mut old_day = DayPlanningResponse::default();
        let mut new_day = DayPlanningResponse::default();

        // Make the slot start in 30 minues, should be ignored if today
        let time_in_30_minutes = chrono::Local::now()
            .naive_local()
            .time()
            .overflowing_add_signed(TimeDelta::minutes(30));

        // Make the slot already passed, should be ignored if today
        let time_30_minutes_ago = chrono::Local::now()
            .naive_local()
            .time()
            .overflowing_sub_signed(TimeDelta::minutes(30));

        // Make the slot in 3 hours, should pass any day
        let time_in_3_hours = chrono::Local::now()
            .naive_local()
            .time()
            .overflowing_add_signed(TimeDelta::minutes(180));

        old_day.add_slot(time_30_minutes_ago.0, false);
        old_day.add_slot(time_in_30_minutes.0, false);
        old_day.add_slot(time_in_3_hours.0, false);
        new_day.add_slot(time_30_minutes_ago.0, true);
        new_day.add_slot(time_in_30_minutes.0, true);
        new_day.add_slot(time_in_3_hours.0, true);

        let mut old_cal = BTreeMap::new();
        let mut new_cal = BTreeMap::new();

        // Insert one today
        let today = chrono::Local::now().format(DATE_FORMAT).to_string();
        old_cal.insert(today.clone(), old_day.clone());
        new_cal.insert(today.clone(), new_day.clone());

        // Insert one tomorrow
        let tomorrow = chrono::Local::now()
            .with_day(chrono::Local::now().day() + 1)
            .unwrap()
            .format(DATE_FORMAT)
            .to_string();
        old_cal.insert(tomorrow.clone(), old_day.clone());
        new_cal.insert(tomorrow.clone(), new_day.clone());

        // Insert one day before
        let yesterday = chrono::Local::now()
            .with_day(chrono::Local::now().day() - 1)
            .unwrap()
            .format(DATE_FORMAT)
            .to_string();
        old_cal.insert(yesterday.clone(), old_day.clone());
        new_cal.insert(yesterday.clone(), new_day.clone());

        // Slots in 1 hour today should not be counted
        let freed = freed_courts(&Availabilities(new_cal), &Availabilities(old_cal));
        // Only the passed slot and the slot in 30 minutes today should be ignored
        assert_eq!(freed.iter().count(), 4);
        assert!(freed.iter().all(|(_, _, _, price)| price.bookable()));
    }

    #[test]
    fn freed_courts_correct() {
        let mut old = crate::mock::real_data_availabilities(2);
        let mut new = old.clone();
        let new_ptr: *const Availabilities = &new as *const Availabilities;
        let old_ptr: *const Availabilities = &old as *const Availabilities;

        let new_avail = freed_courts(&new, &old);
        assert!(new_avail.is_empty());

        let now = chrono::Local::now().naive_local();
        let mut counter = 0;
        for ((date, old_day), (_, new_day)) in old.0.iter_mut().zip(new.0.iter_mut()) {
            for (old_court, new_court) in old_day
                .courts_mut()
                .iter_mut()
                .zip(new_day.courts_mut().iter_mut())
            {
                for (old_slot, new_slot) in old_court
                    .slots_mut()
                    .iter_mut()
                    .zip(new_court.slots_mut().iter_mut())
                {
                    let slot_time_ok = !is_slot_too_soon_or_already_passed(
                        now,
                        date,
                        old_slot,
                        NB_MINUTES_THRESHOLD_FOR_NOTIF,
                    );
                    for (old_price, new_price) in old_slot
                        .prices_mut()
                        .iter_mut()
                        .zip(new_slot.prices_mut().iter_mut())
                    {
                        old_price.set_bookable(false);
                        new_price.set_bookable(true);
                        if slot_time_ok {
                            counter += 1;
                        }
                        unsafe {
                            let avail = freed_courts(&*new_ptr, &*old_ptr);
                            assert_eq!(avail.iter().count(), counter);
                            assert!(avail.iter().all(|(_, _, _, price)| price.bookable()));
                        }
                    }
                }
            }
        }
    }
}

fn is_slot_too_soon_or_already_passed(
    now: chrono::NaiveDateTime,
    slot_date: &str,
    slot: &Slot,
    threshold_in_min: i64,
) -> bool {
    let slot_date =
        chrono::NaiveDate::parse_from_str(slot_date, DATE_FORMAT).expect("Date has wrong format");
    let slot_start = chrono::NaiveTime::parse_from_str(slot.start_at(), TIME_FORMAT)
        .expect("Slot time has wrong format");
    let slot_time = chrono::NaiveDateTime::new(slot_date, slot_start);

    let time_between_slot_and_now = slot_time.signed_duration_since(now);
    time_between_slot_and_now < chrono::Duration::minutes(threshold_in_min)
}

pub fn setup_logging() -> WorkerGuard {
    dotenvy::dotenv().ok();
    let log_dir = std::env::var("LOG_DIRECTORY").expect("LOG_DIRECTORY must be set");

    let file_appender = tracing_appender::rolling::daily(log_dir, "server.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .json()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("trace"))
                .add_directive("hyper=off".parse().unwrap_or_default()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(file_layer)
        .init();
    guard
}
