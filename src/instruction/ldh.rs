use crate::{cpu, util};

pub fn a8_a(cpu: &mut cpu::CPU) {
    let value = cpu.get_register(util::Register::A);
    let addr = cpu.bus.borrow().rom_read_byte(cpu.pc + 1).unwrap();

    let _ = cpu.bus.borrow_mut().rom_write_byte(addr as u16 + 0xFF00, value);

    cpu.pc += 2;
}

pub fn a_a8(cpu: &mut cpu::CPU) {
    let addr = cpu.bus.borrow().rom_read_byte(cpu.pc + 1).unwrap();
    let value = cpu.bus.borrow().rom_read_byte(addr as u16 + 0xFF00).unwrap();

    cpu.set_register(util::Register::A, value);

    cpu.pc += 2;
}
