use crate::emulator::{bus, cpu, disassemble::{Disasm, Operand}, util};


pub fn a8_a_disasm(bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let offset = bus.read_byte(addr + 1).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, offset],
        length: 2,
        mnemonic: format!("LDH ($FF{:02X}), A", offset),
        verb: "LDH".into(),
        operands: vec![Operand::Address(0xFF00 + offset as u16), Operand::Register8("A".into())],
    })
}

pub fn a_a8_disasm(bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let offset = bus.read_byte(addr + 1).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, offset],
        length: 2,
        mnemonic: format!("LDH A, ($FF{:02X})", offset),
        verb: "LDH".into(),
        operands: vec![Operand::Register8("A".into()), Operand::Address(0xFF00 + offset as u16)],
    })
}

pub fn c_a_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "LDH ($FF00+C), A".into(),
        verb: "LDH".into(),
        operands: vec![Operand::Register8("($FF00+C)".into()), Operand::Register8("A".into())],
    })
}

pub fn a_c_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "LDH A, ($FF00+C)".into(),
        verb: "LDH".into(),
        operands: vec![Operand::Register8("A".into()), Operand::Register8("($FF00+C)".into())],
    })
}
