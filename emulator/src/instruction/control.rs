use crate::emulator::bus::Bus;
use crate::emulator::cpu::CPU;

pub fn nop(_cpu: &mut CPU, _bus: &mut Bus) -> (u16, u8) {
    (1, 4)
}

pub fn halt(cpu: &mut CPU, _bus: &mut Bus) -> (u16, u8) {
    cpu.is_halting = true;
    (1, 4)
}

pub fn ei(cpu: &mut CPU, _bus: &mut Bus) -> (u16, u8) {
    cpu.ie_enable_delay = true;
    (1, 4)
}

pub fn di(_cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    bus.interrupts.ime = false;
    (1, 4)
}