use crate::{cpu, util};

pub fn r8(cpu: &mut cpu::CPU, opcode: u8) {
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(util::Register::A);
    let x = cpu.get_register(register);
    let carry_in = if cpu.flags.carry { 1 } else { 0 };

    let (temp, carry1) = a.overflowing_sub(x);
    let (value, carry2) = temp.overflowing_sub(carry_in);

    cpu.set_register(util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = true;
    cpu.flags.half_carry = ((a & 0x0F).wrapping_sub((x & 0x0F) + carry_in)) & 0x10 != 0;
    cpu.flags.carry = carry1 || carry2;

    cpu.pc += 1;
}

