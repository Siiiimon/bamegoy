use crate::bus;
use crate::{cpu, disassemble::Disasm, util};

pub fn a16(cpu: &mut cpu::CPU, opcode: u8) {
    let target = cpu.bus.borrow().read_word(cpu.pc + 1).unwrap();

    let should_jump = match opcode >> 4 {
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump || opcode == 0o303 {
        cpu.pc = target;
    } else {
        cpu.pc += 3;
    }
}

pub fn e8(cpu: &mut cpu::CPU, opcode: u8) {
    let offset = cpu.bus.borrow().read_byte(cpu.pc + 1).unwrap() as i8;
    let target = if offset < 0 {
        cpu.pc.wrapping_add(2).wrapping_sub((-offset) as u16)
    } else {
        cpu.pc.wrapping_add(2).wrapping_add(offset as u16)
    };

    let should_jump = match opcode >> 4 {
        1 => true,
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump {
        cpu.pc = target;
    } else {
        cpu.pc += 2;
    }
}

pub fn hl(cpu: &mut cpu::CPU) {
    let addr = cpu.get_register_pair(util::RegisterPair::HL);
    cpu.pc = addr;
}

pub fn a16_disasm(bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let target = bus.read_word(addr).unwrap();

    let mnemonic = match opcode {
        0xC3 => format!("JP ${:04X}", target),
        0xC2 => format!("JP NZ, ${:04X}", target),
        0xCA => format!("JP Z, ${:04X}", target),
        0xD2 => format!("JP NC, ${:04X}", target),
        0xDA => format!("JP C, ${:04X}", target),
        _ => return None,
    };

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, target as u8, (target >> 8) as u8],
        length: 3,
        mnemonic,
    })
}

pub fn e8_disasm(bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let offset = bus.read_byte(addr + 1).unwrap() as i8;
    let target = if offset < 0 {
        addr.wrapping_add(2).wrapping_sub((-offset) as u16)
    } else {
        addr.wrapping_add(2).wrapping_add(offset as u16)
    };

    let mnemonic = match opcode {
        0x18 => format!("JR ${:04X}", target),
        0x20 => format!("JR NZ, ${:04X}", target),
        0x28 => format!("JR Z, ${:04X}", target),
        0x30 => format!("JR NC, ${:04X}", target),
        0x38 => format!("JR C, ${:04X}", target),
        _ => return None,
    };

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, offset as u8],
        length: 2,
        mnemonic,
    })
}

pub fn hl_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "JP HL".into(),
    })
}
