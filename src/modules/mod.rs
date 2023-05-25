mod utils;
use crate::module::Module;

mod cpu;
mod memory;
mod time;

pub const ALL_MODULES: &[&str] = &["memory", "cpu", "time"];

pub fn get_module(name: &str) -> Option<Box<dyn Module>> {
    match name {
        "time" => Some(Box::new(time::TimeModule::new())),
        "memory" => Some(Box::new(memory::MemoryModule::new())),
        "cpu" => Some(Box::new(cpu::CpuModule::new())),
        _ => None,
    }
}
