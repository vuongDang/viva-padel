use chrono::{Datelike, NaiveDate, NaiveTime};

use crate::{
    models::{Alarm, CourtType, legarden::Availabilities},
    services::legarden::{DATE_FORMAT, TIME_FORMAT},
};

impl Alarm {
    // Filter availabilities depending on the alarm criteria
    pub fn target_availabilities(&self, avail: Availabilities) -> Availabilities {
        let today = chrono::Local::now().date_naive();
        let max_date = today + chrono::Duration::weeks(self.weeks_ahead as i64);

        avail
            .into_iter()
            .filter_map(|(key, day_planning)| {
                // Keep the availabilities which dates are within the alarm week_ahead and match weekday
                let date = match NaiveDate::parse_from_str(&key, DATE_FORMAT) {
                    Ok(d) => d,
                    Err(_) => return None, // If date is invalid, discard this day
                };

                if !(date >= today && date <= max_date) {
                    return None; // Not within the weeks_ahead range
                }

                if !self.days_of_the_week.contains(&date.weekday()) {
                    return None; // Doesn't match alarm's weekdays
                }

                // Filter courts within the day_planning
                let filtered_courts = day_planning
                    .courts()
                    .iter()
                    .filter_map(|court| {
                        // Keep the availabilities which court type match the alarm court_type
                        let court_type_matches = match self.court_type {
                            CourtType::Indoor => court.is_indoor(),
                            CourtType::Outdoor => !court.is_indoor(),
                            CourtType::Both => true,
                        };

                        if !court_type_matches {
                            return None;
                        }

                        // Filter slots within the court
                        let filtered_slots = court
                            .slots()
                            .iter()
                            .filter_map(|slot| {
                                // Keep the availabilities which start time are within the time_range of the alarm
                                let start_time =
                                    NaiveTime::parse_from_str(slot.start_at(), TIME_FORMAT)
                                        .unwrap();
                                let is_within_time_range = start_time >= self.time_range.0
                                    && start_time < self.time_range.1;
                                let new_prices = slot
                                    .prices()
                                    .iter()
                                    .filter(|price| {
                                        let is_bookable = price.bookable();
                                        let does_slot_duration_fit =
                                            self.slot_durations.contains(&price.duration());
                                        is_bookable && does_slot_duration_fit
                                    })
                                    .cloned()
                                    .collect();
                                let new_slot = slot.clone_with_prices(new_prices);
                                (is_within_time_range && !new_slot.prices().is_empty())
                                    .then_some(new_slot)
                            })
                            .collect::<Vec<_>>();

                        if filtered_slots.is_empty() {
                            None
                        } else {
                            Some(court.clone_with(filtered_slots))
                        }
                    })
                    .collect::<Vec<_>>();

                if filtered_courts.is_empty() {
                    None
                } else {
                    Some((key, day_planning.new_with(filtered_courts)))
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{
            Alarm,
            legarden::{DayPlanningResponse, Slot},
        },
        services::legarden::DATE_FORMAT,
    };

    use super::*;
    use chrono::{Datelike, Duration, Local, NaiveTime, Weekday};
    use std::collections::BTreeMap;

    // Helper function to create a comprehensive Availabilities map for testing.
    fn setup_test_availabilities() -> Availabilities {
        let today = Local::now().date_naive();
        // Base everything on the next Monday to ensure all dates are in the future
        let monday_next_week =
            today + Duration::days((7 - today.weekday().num_days_from_monday() as i64) % 7);
        let tuesday_next_week = monday_next_week + Duration::days(1);
        let monday_in_2_weeks = monday_next_week + Duration::weeks(1);
        let monday_in_5_weeks = monday_next_week + Duration::weeks(4);

        let dates = [
            today.format(DATE_FORMAT).to_string(),
            monday_next_week.format(DATE_FORMAT).to_string(),
            tuesday_next_week.format(DATE_FORMAT).to_string(),
            monday_in_2_weeks.format(DATE_FORMAT).to_string(),
            monday_in_5_weeks.format(DATE_FORMAT).to_string(),
        ];

        let simple_day = DayPlanningResponse::simple_day();
        let mut avail = BTreeMap::new();
        for (i, date) in dates.into_iter().enumerate() {
            let mut day = simple_day.clone();
            // make one court at the 14:00 time slot
            if i == 1 {
                let slots = day.courts_mut().first_mut().unwrap().slots_mut();
                let new_slot =
                    Slot::clone_with_start_at(slots.first().unwrap(), "14:00".to_owned());
                slots.push(new_slot);
            }
            // Make one court outside
            if i == 3 {
                day.courts_mut().first_mut().unwrap().set_indoor(false);
            }
            avail.insert(date, day);
        }
        avail
    }

    #[test]
    fn test_filter_by_weekday() {
        let avail = setup_test_availabilities();
        let alarm = Alarm {
            days_of_the_week: vec![Weekday::Tue],
            weeks_ahead: 5, // Set high to not interfere
            ..Default::default()
        };

        let mut result = alarm.target_availabilities(avail);
        assert_eq!(
            result.len(),
            1,
            "Should only find availabilities for Tuesday"
        );

        let (date, _) = result.pop_first().unwrap();
        let weekday = NaiveDate::parse_from_str(&date, DATE_FORMAT)
            .unwrap()
            .weekday();
        assert_eq!(
            weekday,
            Weekday::Tue,
            "The resulting key should be the Tuesday date"
        );
    }

    #[test]
    fn test_filter_by_weeks_ahead() {
        let avail = setup_test_availabilities();
        // Alarm for the next 2 weeks (includes this week and next week)
        let alarm = Alarm {
            weeks_ahead: 2,
            ..Default::default()
        };

        let result = alarm.target_availabilities(avail);
        // Should find this week's Monday, this week's Tuesday, and next week's Monday
        let expected_result = if Local::now().date_naive().weekday() == Weekday::Mon {
            3
        } else {
            4
        };
        assert_eq!(
            result.len(),
            expected_result,
            "Should find 4 days within the next 2 weeks"
        );
    }

    #[test]
    fn test_filter_by_time_range() {
        let avail = setup_test_availabilities();
        let alarm = Alarm {
            // Only look for slots between 13:00 and 15:00
            time_range: (
                NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
            ),
            weeks_ahead: 5, // Set high to not interfere
            ..Default::default()
        };

        let result = alarm.target_availabilities(avail);
        assert_eq!(
            result.len(),
            1,
            "Should only find one day with matching slots"
        );

        let day = result.values().next().unwrap();
        assert_eq!(
            day.courts().len(),
            1,
            "Should be one court with a matching slot"
        );
        assert_eq!(
            day.courts()[0].slots().len(),
            1,
            "Should be one matching slot"
        );
        // assert!(day.courts()[0].slots()[0].start_at().contains("T14:00:00"));
    }

    #[test]
    fn test_filter_by_court_type() {
        let avail = setup_test_availabilities();
        let alarm = Alarm {
            court_type: CourtType::Outdoor,
            weeks_ahead: 5, // Set high to not interfere
            ..Default::default()
        };

        let result = alarm.target_availabilities(avail);
        assert_eq!(
            result.len(),
            1,
            "Should only find one day with an outdoor court"
        );
        let day = result.values().next().unwrap();
        assert_eq!(day.courts().len(), 1, "Should only be one outdoor court");
        assert!(!day.courts()[0].is_indoor());
    }

    #[test]
    fn test_combined_filters() {
        let avail = setup_test_availabilities();
        // Alarm for outdoor courts on Mondays between 09:00 and 11:00, within the next 3 weeks
        let alarm = Alarm {
            days_of_the_week: vec![Weekday::Mon],
            weeks_ahead: 3,
            court_type: CourtType::Outdoor,
            time_range: (
                NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
            ),
            slot_durations: vec![5400, 7200],
            ..Default::default()
        };

        let result = alarm.target_availabilities(avail);
        assert_eq!(
            result.len(),
            1,
            "Should be exactly one day matching all criteria"
        );
        let day = result.values().next().unwrap();
        dbg!(&day);
        assert_eq!(day.courts().len(), 1, "Should be one court");
        assert!(!day.courts()[0].is_indoor(), "Court should be outdoor");
        assert_eq!(day.courts()[0].slots().len(), 1, "Should be one slot");
        assert!(day.courts()[0].slots()[0].start_at().contains("10:00"));
        assert!(day.courts()[0].slots()[0].prices().len() == 1);
    }

    #[test]
    fn test_no_matches() {
        let avail = setup_test_availabilities();
        // Alarm for a time where there are no slots
        let alarm = Alarm {
            time_range: (
                NaiveTime::from_hms_opt(1, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(2, 0, 0).unwrap(),
            ),
            weeks_ahead: 5,
            ..Default::default()
        };

        let result = alarm.target_availabilities(avail);
        assert!(
            result.is_empty(),
            "Result should be empty when no slots match"
        );
    }
}
