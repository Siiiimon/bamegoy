use crate::{cpu, util};

pub fn r8(cpu: &mut cpu::CPU, opcode: u8) {
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(util::Register::A);
    let x = cpu.get_register(register);

    let (value, carry) = a.overflowing_sub(x);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = true;
    cpu.flags.carry = carry;
    cpu.flags.half_carry = (a & 0x0F) < (x & 0x0F);

    cpu.pc += 1;
}
