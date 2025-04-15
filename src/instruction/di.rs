use crate::cpu;

pub fn di(cpu: &mut cpu::CPU) {
    cpu.interrupt_master = false;
    cpu.pc += 1;
}
