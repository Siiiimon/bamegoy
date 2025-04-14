use crate::cpu;

pub fn a16(cpu: &mut cpu::CPU) {
    let lo = cpu.bus.borrow().rom_read_byte(cpu.pc + 1).unwrap();
    let hi = cpu.bus.borrow().rom_read_byte(cpu.pc + 2).unwrap();
    cpu.pc = ((hi as u16) << 8) | lo as u16;
}
