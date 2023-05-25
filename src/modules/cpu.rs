use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::module::{Module, UpdateStatus};

type CpuStats = [u64; 10];

pub struct CpuModule {
    cache: String,
    status: CpuStats,
}

impl CpuModule {
    pub fn new() -> Self {
        Self {
            cache: String::new(),
            status: CpuStats::default(),
        }
    }

    fn get_status() -> Result<CpuStats> {
        let file = File::open("/proc/stat")?;
        let buf = BufReader::new(file);
        if let Some(Ok(line)) = buf.lines().next() {
            let results_vec: Vec<u64> = line
                .split_ascii_whitespace()
                .skip(1)
                .filter_map(|s| s.parse::<u64>().ok())
                .take(10)
                .collect();
            match results_vec.try_into() {
                Ok(m) => Ok(m),
                Err(_) => Err(anyhow!("Error occurred parsing /proc/stat")),
            }
        } else {
            Err(anyhow!("Error occurred while reading /proc/stat."))
        }
    }

    fn get_delta(&mut self) -> Result<f64> {
        let last = self.status;
        let current = Self::get_status()?;
        let delta: Vec<u64> = last
            .iter()
            .zip(current.iter())
            .map(|(x0, x1)| x1 - x0)
            .collect();
        self.status = current;

        let idle_time = delta[3] + delta[4];
        let sum_time: u64 = delta.iter().sum();
        Ok(1.0 - (idle_time as f64) / (sum_time as f64))
    }
}

impl Module for CpuModule {
    fn get_string(&self) -> &str {
        &self.cache
    }

    fn update(&mut self) -> UpdateStatus {
        if let Ok(cpu_load) = self.get_delta() {
            self.cache = format!(" {:.2}%", cpu_load * 100.0);
            UpdateStatus::All
        } else {
            UpdateStatus::None
        }
    }

    fn update_interval(&self) -> u64 {
        1000
    }
}
