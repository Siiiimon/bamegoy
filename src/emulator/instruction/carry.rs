use crate::emulator::bus;
use crate::emulator::{cpu, disassemble::Disasm};

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

