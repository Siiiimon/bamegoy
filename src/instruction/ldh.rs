use crate::{cpu, disassemble::Disasm, util};

pub fn a8_a(cpu: &mut cpu::CPU) {
    let value = cpu.get_register(util::Register::A);
    let addr = cpu.bus.borrow().read_byte(cpu.pc + 1).unwrap();

    let _ = cpu.bus.borrow_mut().write_byte(addr as u16 + 0xFF00, value);

    cpu.pc += 2;
}

pub fn a_a8(cpu: &mut cpu::CPU) {
    let addr = cpu.bus.borrow().read_byte(cpu.pc + 1).unwrap();
    let value = cpu.bus.borrow().read_byte(addr as u16 + 0xFF00).unwrap();

    cpu.set_register(util::Register::A, value);

    cpu.pc += 2;
}

pub fn c_a(cpu: &mut cpu::CPU) {
    let value = cpu.get_register(util::Register::A);
    let offset = cpu.get_register(util::Register::C);

    let _ = cpu.bus.borrow_mut().write_byte(0xFF00 + offset as u16, value);

    cpu.pc += 1;
}

pub fn a_c(cpu: &mut cpu::CPU) {
    let offset = cpu.get_register(util::Register::C);
    let value = cpu.bus.borrow().read_byte(0xFF00 + offset as u16).unwrap();

    cpu.set_register(util::Register::A, value);

    cpu.pc += 1;
}

pub fn a8_a_disasm(mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    let offset = *mem.get((addr + 1) as usize)?;

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, offset],
        length: 2,
        mnemonic: format!("LDH ($FF{:02X}), A", offset),
    })
}

pub fn a_a8_disasm(mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    let offset = *mem.get((addr + 1) as usize)?;

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, offset],
        length: 2,
        mnemonic: format!("LDH A, ($FF{:02X})", offset),
    })
}

pub fn c_a_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "LDH ($FF00+C), A".into(),
    })
}

pub fn a_c_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "LDH A, ($FF00+C)".into(),
    })
}
