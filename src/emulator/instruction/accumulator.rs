use crate::emulator::bus;
use crate::emulator::{cpu, disassemble::Disasm, util};

pub fn cpl_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "CPL".into(),
        verb: "CPL".into(),
        operands: vec![],
    })
}

pub fn daa_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "DAA".into(),
        verb: "DAA".into(),
        operands: vec![],
    })
}
