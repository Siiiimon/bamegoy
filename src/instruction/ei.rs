use crate::cpu;

pub fn ei(cpu: &mut cpu::CPU) {
    cpu.ie_enable_delay = true;
    cpu.pc += 1;
}
