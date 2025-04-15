use crate::{cpu, util};

pub fn r8(cpu: &mut cpu::CPU, opcode: u8) {
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(util::Register::A);
    let x = cpu.get_register(register);

    let value = x.wrapping_add(a);
    cpu.set_register(util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) > 0x0F;

    cpu.pc += 1;
}

pub fn a_n8(cpu: &mut cpu::CPU) {
    let a = cpu.get_register(util::Register::A);
    let x = cpu.bus.borrow().rom_read_byte(cpu.pc + 1).unwrap();

    let value = a.wrapping_add(x);
    cpu.set_register(util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) > 0x0F;

    cpu.pc += 1;
}

pub fn r16(cpu: &mut cpu::CPU, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let hl = cpu.get_register_pair(util::RegisterPair::HL);
    let xy = cpu.get_register_pair(pair);

    let (value, carry) = hl.overflowing_add(xy);
    cpu.set_register_pair(util::RegisterPair::HL, value);

    cpu.flags.subtraction = false;
    cpu.flags.carry = carry;
    cpu.flags.half_carry = ((hl & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF;

    cpu.pc += 1;
}
