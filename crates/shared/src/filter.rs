use crate::{CLOSING_TIME, OPENING_TIME};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A filter used to specify which padel courts we want to book
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct Filter {
    pub name: String,
    pub days_of_the_week: Vec<String>,
    pub start_time_slots: Vec<(String, String)>,
    pub with_outdoor: bool,
}

impl Default for Filter {
    /// Default `Filter` that allows everything
    fn default() -> Self {
        Filter {
            name: "default".to_string(),
            days_of_the_week: vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
                .into_iter()
                .map(|day| day.into())
                .collect(),
            start_time_slots: vec![(OPENING_TIME.into(), CLOSING_TIME.into())],
            with_outdoor: true,
        }
    }
}

impl PartialEq for Filter {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Filter {
    pub fn default_filters() -> HashMap<String, Filter> {
        let mut filters = HashMap::new();
        filters.insert("default".to_string(), Filter::default());
        filters
    }
}
