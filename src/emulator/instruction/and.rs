use crate::emulator::bus;
use crate::emulator::disassemble::Operand;
use crate::emulator::{cpu, disassemble::Disasm, util};

pub fn r8_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let register = util::get_register_by_code(opcode & 0b111);

    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("AND {}", register),
        verb: "AND".to_string(),
        operands: vec![Operand::Register8(register.to_string())],
    })
}

pub fn a_n8_disasm(bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let imm = bus.read_byte(addr + 1).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, imm],
        length: 2,
        mnemonic: format!("AND ${:02X}", imm),
        verb: "AND".to_string(),
        operands: vec![Operand::Immediate8(imm)],
    })
}

