pub mod errors;
pub mod filter;
pub mod models;
pub mod pull_data_from_garden;
pub mod utils;

/// Day format that the server uses
pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const TIME_FORMAT: &str = "%H:%M";

pub const OPENING_TIME: &str = "09:00";
pub const CLOSING_TIME: &str = "22:00";

pub const DAYS_PER_WEEK: u8 = 7;
pub const NB_WEEKS_SHOWN: u8 = 4;
pub const NB_DAYS_PER_BATCH: u8 = DAYS_PER_WEEK * NB_WEEKS_SHOWN;
