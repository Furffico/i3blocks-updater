use super::utils::{extract_from_file, format_bytes};
use crate::module::{Module, UpdateStatus};

pub struct MemoryModule {
    cache: String,
    results: [i64; 4],
}

impl MemoryModule {
    const QUERIES: [&str; 4] = ["MemTotal:", "MemFree:", "Buffers:", "Cached:"];

    pub fn new() -> Self {
        Self {
            cache: String::new(),
            results: [0; 4],
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
        // let used_ratio: f32 = 1.0 - (self.results[1] as f32) / (self.results[0] as f32);
        // self.cache = format!("Memory: {:.3}%", 100.0 * used_ratio);
        let memused = (self.results[0] - self.results[1] - self.results[2] - self.results[3]) << 10;
        self.cache = format!(" {}", format_bytes(memused as u64));
        status
    }
    fn update_interval(&self) -> u64 {
        2000
    }
}
