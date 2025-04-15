use crate::cpu;

pub fn scf(cpu: &mut cpu::CPU) {
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = true;

    cpu.pc += 1;
}

pub fn ccf(cpu: &mut cpu::CPU) {
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = !cpu.flags.carry;

    cpu.pc += 1;
}
