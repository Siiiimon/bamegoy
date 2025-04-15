use crate::{cpu, util};

pub fn cpl(cpu: &mut cpu::CPU) {
    let value = cpu.get_register(util::Register::A);
    cpu.set_register(util::Register::A, !value);

    cpu.flags.subtraction = true;
    cpu.flags.half_carry = true;

    cpu.pc += 1;
}
