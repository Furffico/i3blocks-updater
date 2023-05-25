use std::env;
use std::thread;
use std::time::{Duration, Instant};

mod module;
mod modules;
mod printer;
mod schedule;

use module::Module;
use modules::{get_module, ALL_MODULES};
use printer::Printer;
use schedule::Schedule;

fn main() {
    /* ------------------------------- module list ------------------------------ */
    let mut modules: Vec<Box<dyn Module>> = {
        let mut modules = Vec::new();
        let og_args: Vec<String> = env::args().collect();
        let args = if og_args.len() <= 1 {
            ALL_MODULES.to_vec()
        } else {
            og_args.iter().skip(1).map(|v| v.as_str()).collect()
        };
        for name in args {
            if let Some(m) = get_module(name) {
                modules.push(m);
            } else {
                panic!("Module {} not found!", name);
            }
        }
        if modules.is_empty() {
            panic!("No module is selected!");
        }
        modules
    };

    /* --------------------------------- printer -------------------------------- */
    let mut printer = Printer::new();

    /* -------------------------------- schedule -------------------------------- */
    let mut schedule: Schedule = {
        let mut sc = Schedule::new();
        let t = Instant::now();
        // pending initial jobs
        for i in 0..modules.len() {
            sc.push_job(t, i);
        }
        sc
    };

    /* -------------------------------- mainloop -------------------------------- */
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
