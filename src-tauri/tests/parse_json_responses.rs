use shared::frontend::calendar_ui::*;
use shared::server_structs::*;

#[test]
fn parse_and_convert_json_planning_response() {
    let response = testcases::json_planning_for_1_day();
    let parsed = serde_json::from_str::<DayPlanningResponse>(&response);
    assert!(parsed.is_ok());
    let day_planning: DayPlanning = parsed.unwrap().into();
    let slot_at_1600 = day_planning.slots.get("16:00");
    let padel4 = PadelCourt {
        name: "Padel 4".into(),
        is_indoor: true,
    };

    let padel5 = PadelCourt {
        name: "Padel 5".into(),
        is_indoor: true,
    };

    assert!(slot_at_1600.is_some());
    assert!(slot_at_1600.unwrap().available_courts.contains(&padel4));
    assert!(!slot_at_1600.unwrap().available_courts.contains(&padel5));
}

// This case is special as the server gave us Padel courts with field `bookable: false`
// This does not seem the case with the official app but currently we will take the information as
// it is and assume that there are no available courts
#[test]
fn parse_and_convert_json_planning_response_on_failed_case() {
    let response = testcases::json_planning_for_1_day_by_filename("day (10).json");
    let parsed = serde_json::from_str::<DayPlanningResponse>(&response);
    assert!(parsed.is_ok());
    let day_planning: DayPlanning = parsed.unwrap().into();
    assert!(day_planning.slots.is_empty())
}
