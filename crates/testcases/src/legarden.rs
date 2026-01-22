use std::{path::PathBuf, str::FromStr};

pub fn json_planning_for_1_day() -> String {
    let mut path = path_to_legarden_mock_data();
    path.push("formatted_day.json");
    std::fs::read_to_string(path).expect("Error while getting data")
}

pub fn json_planning_for_1_day_by_filename(file: &str) -> String {
    let mut path = path_to_legarden_mock_data();
    path.push(file);
    std::fs::read_to_string(path).expect("Error while getting data")
}

pub fn json_planning_for_29_days() -> Vec<String> {
    let mut calendar = vec![];
    let path = path_to_legarden_mock_data();
    for i in 0..=28 {
        let day_path = path.join(format!("day({i}).json"));
        let content = std::fs::read_to_string(day_path).expect("Error while getting data");
        calendar.push(content)
    }
    calendar
}

pub fn json_planning_simple_day() -> String {
    let mut path = path_to_legarden_mock_data();
    path.push("simple_day.json");
    std::fs::read_to_string(path).expect("Error while getting data")
}

pub fn json_planning_simple_all_booked() -> String {
    let mut path = path_to_legarden_mock_data();
    path.push("simple_day_all_booked.json");
    std::fs::read_to_string(path).expect("Error while getting data")
}

// If PADEL_MOCK_DATA is set, pick the path from it else
fn path_to_legarden_mock_data() -> PathBuf {
    const PATH: [&'static str; 4] = ["crates", "testcases", "data", "legarden_server"];
    if let Ok(path) = std::env::var("PADEL_MOCK_DATA") {
        PathBuf::from_str(&path)
            .expect("Failed to find mock data directory")
            .join("legarden_server")
    } else {
        let mut path = PathBuf::from(std::env!("CARGO_WORKSPACE_DIR"));
        for path_element in PATH.iter() {
            path.push(path_element);
        }
        path
    }
}
