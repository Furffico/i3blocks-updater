use super::utils::extract_from_file;
use crate::module::{Module, UpdateStatus};

pub struct MemoryModule {
    cache: String,
    results: [i64; 2],
}

impl MemoryModule {
    const QUERIES: [&str; 2] = ["MemTotal", "MemFree"];

    pub fn new() -> Self {
        Self {
            cache: String::new(),
            results: [0, 0],
        }
    }

    fn get_results(&mut self) -> UpdateStatus {
        match extract_from_file("/proc/meminfo", &Self::QUERIES, &mut self.results) {
            Ok(s) => s,
            Err(_) => UpdateStatus::None,
        }
    }
}

impl Module for MemoryModule {
    fn get_string(&self) -> &str {
        &self.cache
    }

    fn update(&mut self) -> UpdateStatus {
        let status = self.get_results();
        if !status.bool() {
            return status;
        }
        let used_ratio: f32 = 1.0 - (self.results[1] as f32) / (self.results[0] as f32);
        self.cache = format!("Memory: {:.3}%", 100.0 * used_ratio);
        status
    }
    fn update_interval(&self) -> u64 {
        2000
    }
}
