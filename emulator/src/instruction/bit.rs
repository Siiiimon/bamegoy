use crate::emulator::{bus::Bus, cpu::CPU, util};

pub fn rlca(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let value = cpu.get_register(bus, util::Register::A);

    cpu.set_register(bus, util::Register::A, value.rotate_left(1));

    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = value & 0x80 != 0;

    (1, 4)
}

pub fn rrca(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let value = cpu.get_register(bus, util::Register::A);

    cpu.set_register(bus, util::Register::A, value.rotate_right(1));

    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = value & 1 != 0;

    (1, 4)
}

pub fn rla(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let value = cpu.get_register(bus, util::Register::A);

    cpu.set_register(bus, util::Register::A, (value << 1) | (cpu.flags.carry as u8));

    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = value & 0x80 != 0;

    (1, 4)
}

pub fn rra(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let value = cpu.get_register(bus, util::Register::A);

    cpu.set_register(bus, util::Register::A, (value >> 1) | ((cpu.flags.carry as u8) << 7));

    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = value & 0x01 != 0;

    (1, 4)
}