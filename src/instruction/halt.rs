use crate::cpu;

pub fn halt(cpu: &mut cpu::CPU) {
    cpu.is_halting = true;
}
