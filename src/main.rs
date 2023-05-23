use anyhow::Result;
use chrono::Local;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::{Duration, Instant};

enum UpdateStatus {
    All,
    Some,
    None,
}

impl UpdateStatus {
    fn bool(&self) -> bool {
        match self {
            Self::All | Self::Some => true,
            Self::None => false,
        }
    }
}

impl From<UpdateStatus> for bool {
    fn from(val: UpdateStatus) -> Self {
        val.bool()
    }
}

trait Module {
    fn get_string(&self) -> &str;
    fn update(&mut self) -> UpdateStatus;
    fn update_interval(&self) -> u64;
}

fn extract_from_file(path: &str, queries: &[&str], results: &mut [i64]) -> Result<UpdateStatus> {
    let file = File::open(path)?;
    let mut q_count = queries.len();
    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(s) => s,
            Err(_) => break,
        };
        for (i, q) in queries.iter().enumerate() {
            if line.starts_with(q) {
                results[i] = match line.split_ascii_whitespace().nth(1) {
                    Some(num_str) => match num_str.parse::<i64>() {
                        Ok(n) => n,
                        Err(_) => continue,
                    },
                    None => continue,
                };
                q_count -= 1;
                if q_count == 0 {
                    return Ok(UpdateStatus::All);
                }
            }
        }
    }
    if q_count == queries.len() {
        Ok(UpdateStatus::None)
    } else {
        Ok(UpdateStatus::Some)
    }
}

/* -------------------------------- RamModule ------------------------------- */
struct RamModule {
    cache: String,
    results: [i64; 2],
}

impl RamModule {
    const QUERIES: [&str; 2] = ["MemTotal", "MemFree"];

    fn new() -> Self {
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

impl Module for RamModule {
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

/* ------------------------------- TimeModule ------------------------------- */
struct TimeModule(String);

impl TimeModule {
    fn new() -> Self {
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

/* ----------------------------------- Job ---------------------------------- */
struct Job {
    id: u64,
    timestamp: Instant,
    module_index: usize,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.timestamp.cmp(&other.timestamp) {
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => Ordering::Greater,
        }
    }
}

/* -------------------------------- Schedule -------------------------------- */
struct Schedule {
    queue: BinaryHeap<Job>,
    count: u64,
}

impl Schedule {
    fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            count: 0,
        }
    }

    fn push_job(&mut self, timestamp: Instant, module_index: usize) {
        self.count += 1;
        self.queue.push(Job {
            id: self.count,
            timestamp,
            module_index,
        })
    }

    fn pop(&mut self) -> Option<Job> {
        self.queue.pop()
    }

    fn next_timestamp(&self) -> Option<Instant> {
        self.queue.peek().map(|j| j.timestamp)
    }
}

/* --------------------------------- Printer -------------------------------- */
struct Printer {
    require_refresh: bool,
    last_refresh: Instant,
}

impl Printer {
    const INTERVAL: Duration = Duration::from_millis(200);

    fn new() -> Self {
        Self {
            require_refresh: true,
            last_refresh: Instant::now(),
        }
    }
    fn require_refresh(&mut self, next_time: Option<Instant>, required: bool) -> bool {
        if required {
            self.require_refresh = true;
        }

        let now = Instant::now();

        if now.duration_since(self.last_refresh) > Self::INTERVAL {
            true
        } else if self.require_refresh {
            match next_time {
                Some(t) => t.saturating_duration_since(now) > Self::INTERVAL,
                None => true,
            }
        } else {
            false
        }
    }

    fn output(&mut self, modules: &Vec<Box<dyn Module>>) {
        let mut s = String::new();
        for m in modules {
            s.push_str(m.get_string());
            s.push(' ');
        }
        println!("{}", s);
        self.require_refresh = false;
        self.last_refresh = Instant::now();
    }
}

/* ---------------------------------- main ---------------------------------- */

fn main() {
    let mut modules: Vec<Box<dyn Module>> =
        vec![Box::new(RamModule::new()), Box::new(TimeModule::new())];
    let t = Instant::now();
    let mut schedule: Schedule = Schedule::new();
    let mut printer = Printer::new();

    for i in 0..modules.len() {
        schedule.push_job(t, i);
    }

    for _ in 0..100 {
        let job = match schedule.pop() {
            Some(j) => j,
            None => break,
        };
        let target_time = job.timestamp;
        let module = match modules.get_mut(job.module_index) {
            Some(m) => m,
            None => continue,
        };

        // wait till execution
        let now = Instant::now();
        if target_time > now {
            let duration = target_time.duration_since(now);
            thread::sleep(duration);
        };

        // let now = Instant::now();
        // println!("Jobid: {} Target: {:?} now: {:?}", job.id, target_time, now);

        // execute job
        let require_refresh: bool = module.update().into();
        let interval = Duration::from_millis(module.update_interval());

        // check if refresh is required
        if printer.require_refresh(schedule.next_timestamp(), require_refresh) {
            printer.output(&modules);
        }

        let next_time = target_time + interval;
        let next_time = if next_time < now {
            now + interval
        } else {
            next_time
        };

        // append job
        schedule.push_job(next_time, job.module_index);
    }
}
