use crate::emulator::runtime::bus::Bus;
use crate::emulator::runtime::cpu::CPU;

pub type Policy = Box<dyn FnMut(&CPU, &Bus) -> bool + Send>;

pub fn single_step() -> Policy {
    Box::new(move |_cpu: &CPU, _bus: &Bus| true)
}
