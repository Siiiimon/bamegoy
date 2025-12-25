use crate::emulator::bus;
use crate::emulator::{cpu, disassemble::Disasm};

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

pub fn scf_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "SCF".into(),
        verb: "SCF".into(),
        operands: vec![],
    })
}

pub fn ccf_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "CCF".into(),
        verb: "CCF".into(),
        operands: vec![],
    })
}
