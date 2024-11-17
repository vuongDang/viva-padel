use std::path::PathBuf;

pub fn json_planning_for_1_day() -> String {
    let mut path = PathBuf::from(std::env!("CARGO_WORKSPACE_DIR"));
    for path_element in [
        "crates",
        "testcases",
        "data",
        "json_server_responses",
        "plannings",
        "formatted_day.json",
    ]
    .iter()
    {
        path.push(path_element);
    }
    std::fs::read_to_string(path).expect("Error while getting data")
}

pub fn json_planning_for_1_day_by_filename(file: &str) -> String {
    let mut path = PathBuf::from(std::env!("CARGO_WORKSPACE_DIR"));
    for path_element in [
        "crates",
        "testcases",
        "data",
        "json_server_responses",
        "plannings",
        file,
    ]
    .iter()
    {
        path.push(path_element);
    }
    std::fs::read_to_string(path).expect("Error while getting data")
}

pub fn json_planning_for_29_days() -> Vec<String> {
    let mut calendar = vec![];
    for i in 0..=28 {
        let mut path = PathBuf::from(std::env!("CARGO_WORKSPACE_DIR"));
        for path_element in [
            "crates",
            "testcases",
            "data",
            "json_server_responses",
            "plannings",
        ]
        .iter()
        {
            path.push(path_element);
        }
        path.push(format!("day ({i}).json"));
        let content = std::fs::read_to_string(path).expect("Error while getting data");
        calendar.push(content)
    }
    calendar
}
