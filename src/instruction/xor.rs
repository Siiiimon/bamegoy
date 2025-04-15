use crate::{cpu, util};

pub fn r8(cpu: &mut cpu::CPU, opcode: u8) {
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(util::Register::A);
    let x = cpu.get_register(register);

    let result = x ^ a;
    cpu.set_register(util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = false;

    cpu.pc += 1;
}

pub fn a_n8(cpu: &mut cpu::CPU) {
    let a = cpu.get_register(util::Register::A);
    let x = cpu.bus.borrow().rom_read_byte(cpu.pc + 1).unwrap();

    let result = x ^ a;
    cpu.set_register(util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = false;

    cpu.pc += 2;
}
