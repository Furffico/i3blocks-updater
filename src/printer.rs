use super::module::Module;
use std::time::{Duration, Instant};

pub struct Printer {
    require_refresh: bool,
    time_out_of_sync: Instant,
}

impl Printer {
    const INTERVAL: Duration = Duration::from_millis(100);

    pub fn new() -> Self {
        Self {
            require_refresh: true,
            time_out_of_sync: Instant::now(),
        }
    }
    pub fn require_refresh(&mut self, next_time: Option<Instant>, required: bool) -> bool {
        let now = Instant::now();
        if required {
            if !self.require_refresh {
                self.time_out_of_sync = now;
            }
            self.require_refresh = true;
        }

        if now.duration_since(self.time_out_of_sync) > Self::INTERVAL {
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

    pub fn output(&mut self, modules: &Vec<Box<dyn Module>>) {
        let mut s = String::new();
        for m in modules {
            s.push_str(m.get_string());
            s.push(' ');
        }
        println!("{}", s);
        self.require_refresh = false;
    }
}
