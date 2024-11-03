pub mod app_structs;
pub mod errors;
pub mod server_structs;
mod tauri_invokes;
mod testcases;

/// This is the day format that the server uses
pub const DAY_FORMAT: &str = "%Y-%m-%d";
pub const TIME_FORMAT: &str = "%H:%M";

pub const OPENING_TIME: &str = "09:00";
pub const CLOSING_TIME: &str = "22:00";
