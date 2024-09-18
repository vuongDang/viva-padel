use viva_padel_lib::server_structs::*;

#[test]
fn parse_get_planning_response() {
    let path = "tests/json_responses/get_planning.json";
    let response = std::fs::read_to_string(path).expect("Failed to read json file");
    let parsed = serde_json::from_str::<PlanningResponse>(&response);
    println!("{:#?}", parsed);
    assert!(parsed.is_ok());
}
