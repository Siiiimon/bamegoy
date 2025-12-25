use crate::emulator::bus::Bus;
use crate::emulator::cpu::CPU;

pub type Policy = Box<dyn FnMut(&CPU, &Bus) -> bool + Send>;

pub fn single_step() -> Policy {
    Box::new(move |_cpu: &CPU, _bus: &Bus| true)
}
