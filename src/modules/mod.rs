mod utils;
use crate::module::Module;

mod memory;
mod time;

// pub const ALL_MODULES: &[&str] = &[
//     "time",
//     "memory",
// ];

pub fn get_module(name: &str) -> Option<Box<dyn Module>> {
    match name {
        "time" => Some(Box::new(time::TimeModule::new())),
        "memory" => Some(Box::new(memory::MemoryModule::new())),
        _ => None,
    }
}
