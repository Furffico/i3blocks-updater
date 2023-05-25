use std::env;
use std::thread;
use std::time::{Duration, Instant};

mod module;
mod modules;
mod printer;
mod schedule;

use module::Module;
use modules::get_module;
use printer::Printer;
use schedule::Schedule;

fn main() {
    let mut modules: Vec<Box<dyn Module>> = Vec::new();
    for name in env::args().skip(1) {
        if let Some(m) = get_module(&name) {
            modules.push(m);
        } else {
            panic!("Module {} not found!", name);
        }
    }
    if modules.is_empty() {
        panic!("No module is selected!")
    }

    let t = Instant::now();
    let mut schedule: Schedule = Schedule::new();
    let mut printer = Printer::new();

    for i in 0..modules.len() {
        schedule.push_job(t, i);
    }

    while let Some(job) = schedule.pop() {
        let target_time = job.timestamp;
        let module = match modules.get_mut(job.module_index) {
            Some(m) => m,
            None => break,
        };

        // wait till execution
        let now = Instant::now();
        if target_time > now {
            let duration = target_time.duration_since(now);
            thread::sleep(duration);
        };

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
