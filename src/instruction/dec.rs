use crate::{cpu, util};

pub fn r8(cpu: &mut cpu::CPU, opcode: u8) {
    let register_code = (opcode >> 3) & 0b111;
    let register = util::get_register_by_code(register_code);

    let current = cpu.get_register(register);
    let new = current.wrapping_sub(1);

    cpu.flags.zero = new == 0;
    cpu.flags.subtraction = true;
    cpu.flags.half_carry = (current & 0x0F) == 0;
    cpu.flags.carry = current == u8::MAX;

    cpu.set_register(register, new);

    cpu.pc += 1;
}
