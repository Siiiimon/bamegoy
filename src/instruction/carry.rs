use crate::{cpu, disassemble::Disasm};

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

pub fn scf_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "SCF".into(),
    })
}

pub fn ccf_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "CCF".into(),
    })
}

