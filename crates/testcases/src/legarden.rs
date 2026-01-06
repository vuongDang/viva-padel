use std::path::PathBuf;

const PATH: [&'static str; 4] = ["crates", "testcases", "data", "legarden_server"];

pub fn json_planning_for_1_day() -> String {
    let mut path = PathBuf::from(std::env!("CARGO_WORKSPACE_DIR"));
    for path_element in PATH.iter() {
        path.push(path_element);
    }
    path.push("formatted_day.json");
    std::fs::read_to_string(path).expect("Error while getting data")
}

pub fn json_planning_for_1_day_by_filename(file: &str) -> String {
    let mut path = PathBuf::from(std::env!("CARGO_WORKSPACE_DIR"));
    for path_element in PATH.iter() {
        path.push(path_element);
    }
    path.push(file);
    std::fs::read_to_string(path).expect("Error while getting data")
}

pub fn json_planning_for_29_days() -> Vec<String> {
    let mut calendar = vec![];
    for i in 0..=28 {
        let mut path = PathBuf::from(std::env!("CARGO_WORKSPACE_DIR"));
        for path_element in PATH.iter() {
            path.push(path_element);
        }
        path.push(format!("day({i}).json"));
        let content = std::fs::read_to_string(path).expect("Error while getting data");
        calendar.push(content)
    }
    calendar
}
