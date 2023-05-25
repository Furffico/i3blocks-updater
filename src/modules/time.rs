use crate::module::{Module, UpdateStatus};
use chrono::Local;

pub struct TimeModule(String);

impl TimeModule {
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl Module for TimeModule {
    fn get_string(&self) -> &str {
        &self.0
    }
    fn update(&mut self) -> UpdateStatus {
        let now = Local::now();
        self.0 = format!("{:}", now);
        UpdateStatus::All
    }
    fn update_interval(&self) -> u64 {
        1000
    }
}
