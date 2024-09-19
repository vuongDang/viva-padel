use shared::app_structs::*;
use shared::server_structs::*;

#[test]
fn parse_and_convert_json_planning_response() {
    let path = "tests/json_responses/get_planning.json";
    let response = std::fs::read_to_string(path).expect("Failed to read json file");
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
