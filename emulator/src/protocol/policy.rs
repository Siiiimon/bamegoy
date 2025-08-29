use crate::bus::Bus;
use crate::cpu::CPU;

pub type Policy = Box<dyn FnMut(&CPU, &Bus) -> bool + Send>;

pub fn single_step() -> Policy {
    Box::new(move |_cpu: &CPU, _bus: &Bus| { true })
}