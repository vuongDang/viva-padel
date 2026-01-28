mod alarm_logic;
pub mod api;
pub mod auth;
#[cfg(feature = "local_dev")]
pub mod mock;
pub mod models;
pub mod services;

use crate::{
    models::legarden::{Availabilities, Court, DayPlanningResponse, Slot},
    services::legarden::DATE_FORMAT,
};
use chrono::{Datelike, Month, Timelike, Weekday};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::time::sleep;
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
pub fn freed_courts(new: &Availabilities, old: &Availabilities) -> Availabilities {
    let mut freed = BTreeMap::new();
    let dates = old.keys().filter(|date| new.contains_key(*date));
    for day_str in dates {
        if let Some(new_day) = new.get(day_str) {
            let old_day = old.get(day_str).unwrap();
            let mut courts = vec![];
            for (old_court, new_court) in old_day.courts().iter().zip(new_day.courts().iter()) {
                let mut slots = vec![];
                for (old_slot, new_slot) in old_court.slots().iter().zip(new_court.slots().iter()) {
                    let mut prices = vec![];
                    for (old_price, new_price) in
                        old_slot.prices().iter().zip(new_slot.prices().iter())
                    {
                        if !old_price.bookable() && new_price.bookable() {
                            prices.push(new_price.clone());
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
    for (user_id, triggers) in avail_filtered_with_alarms {
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
        let alarm_names: Vec<String> = triggers.keys().cloned().collect();
        let availabilities = triggers.values().cloned().collect();
        let title = "Courts libérés! 🎾";
        let body = message_from_availabilities(&availabilities);
        dbg!(&body);

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
    messages.join(", ")
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

    use chrono::NaiveTime;
    use testcases::legarden::{json_planning_simple_all_booked, json_planning_simple_day};

    use crate::{
        mock::simple_availabilities,
        models::{
            CourtType,
            legarden::{DayPlanningResponse, Price},
        },
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
        let old = simple_availabilities(2, json_planning_simple_all_booked());
        let new = simple_availabilities(2, json_planning_simple_day());

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

        let mut old_price = Price::default();
        old_price.set_bookable(false);
        let mut old_slot = Slot::default();
        let mut old_court = Court::default();

        let mut new_price = Price::default();
        new_price.set_bookable(true);
        let mut new_slot = Slot::default();
        let mut new_court = Court::default();

        old_slot.prices_mut().push(old_price);
        old_court.slots_mut().push(old_slot);
        old_day.courts_mut().push(old_court);

        new_slot.prices_mut().push(new_price);
        new_court.slots_mut().push(new_slot);
        new_day.courts_mut().push(new_court);

        let mut old_cal = BTreeMap::new();
        let mut new_cal = BTreeMap::new();
        old_cal.insert("toto".to_owned(), old_day);
        new_cal.insert("toto".to_owned(), new_day);

        let freed = freed_courts(&Availabilities(new_cal), &Availabilities(old_cal));
        assert_eq!(freed.len(), 1);
    }

    #[test]
    fn freed_courts_correct() {
        let mut old = crate::mock::real_data_availabilities(4);
        let mut new = old.clone();
        let new_ptr: *const Availabilities = &new as *const Availabilities;
        let old_ptr: *const Availabilities = &old as *const Availabilities;

        let new_avail = freed_courts(&new, &old);
        assert!(new_avail.is_empty());

        let mut counter = 0;
        for (old_day, new_day) in old.0.values_mut().zip(new.0.values_mut()) {
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
                    for (old_price, new_price) in old_slot
                        .prices_mut()
                        .iter_mut()
                        .zip(new_slot.prices_mut().iter_mut())
                    {
                        old_price.set_bookable(false);
                        new_price.set_bookable(true);
                        counter += 1;
                        unsafe {
                            let avail = freed_courts(&*new_ptr, &*old_ptr);
                            assert_eq!(get_available_prices(&avail).len(), counter);
                        }
                    }
                }
            }
        }
    }

    fn get_available_prices(avail: &Availabilities) -> Vec<Price> {
        let mut prices = vec![];
        avail.values().for_each(|day| {
            day.courts().iter().for_each(|court| {
                court.slots().iter().for_each(|slot| {
                    slot.prices().iter().for_each(|price| {
                        if price.bookable() {
                            prices.push(price.clone());
                        }
                    })
                })
            })
        });
        prices
    }
}
