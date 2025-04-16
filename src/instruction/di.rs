use crate::cpu;

pub fn di(cpu: &mut cpu::CPU) {
    cpu.bus.borrow_mut().io.interrupts.ime = false;
    cpu.pc += 1;
}
