use crate::emulator::bus;
use crate::emulator::{cpu, disassemble::Disasm, util};

pub fn rlca(cpu: &mut cpu::CPU) {
    let value = cpu.get_register(util::Register::A);

    cpu.set_register(util::Register::A, value.rotate_left(1));

    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = value & 0x80 != 0;

    cpu.pc += 1;
}

pub fn rla(cpu: &mut cpu::CPU) {
    let value = cpu.get_register(util::Register::A);

    cpu.set_register(util::Register::A, (value << 1) | (cpu.flags.carry as u8));

    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = value & 0x80 != 0;

    cpu.pc += 1;
}

pub fn rra(cpu: &mut cpu::CPU) {
    let value = cpu.get_register(util::Register::A);

    cpu.set_register(util::Register::A, (value >> 1) | ((cpu.flags.carry as u8) << 7));

    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = value & 0x01 != 0;

    cpu.pc += 1;
}

pub fn rrca(cpu: &mut cpu::CPU) {
    let value = cpu.get_register(util::Register::A);

    cpu.set_register(util::Register::A, value.rotate_right(1));

    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = value & 1 != 0;

    cpu.pc += 1;
}

pub fn rlca_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "RLCA".into(),
        verb: "RLCA".into(),
        operands: vec![],
    })
}

pub fn rla_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "RLA".into(),
        verb: "RLA".into(),
        operands: vec![],
    })
}

pub fn rra_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "RRA".into(),
        verb: "RRA".into(),
        operands: vec![],
    })
}

pub fn rrca_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "RRCA".into(),
        verb: "RRCA".into(),
        operands: vec![],
    })
}
