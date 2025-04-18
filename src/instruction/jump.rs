use crate::bus;
use crate::disassemble::Operand;
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
    let target = bus.read_word(addr + 1).unwrap();

    let instr = match opcode {
        0xC3 => vec!["JP".to_string(), "".to_string()],
        0xC2 => vec!["JP".to_string(), "NZ".to_string()],
        0xCA => vec!["JP".to_string(), "Z".to_string()],
        0xD2 => vec!["JP".to_string(), "NC".to_string()],
        0xDA => vec!["JP".to_string(), "C".to_string()],
        _ => return None,
    };

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, target as u8, (target >> 8) as u8],
        length: 3,
        mnemonic: instr.join(" ").to_string() + " " + &format!("{:04X}", target),
        verb: instr[0].clone(),
        operands: if instr[1].is_empty() {
            vec![Operand::Address(target)]
        } else {
            vec![Operand::Conditional(instr[1].clone()), Operand::Address(target)]
        }
    })
}

pub fn e8_disasm(bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let offset = bus.read_byte(addr + 1).unwrap() as i8;
    let target = if offset < 0 {
        addr.wrapping_add(2).wrapping_sub((-offset) as u16)
    } else {
        addr.wrapping_add(2).wrapping_add(offset as u16)
    };

    let instr = match opcode {
        0x18 => vec!["JP".to_string(), "".to_string()],
        0x20 => vec!["JP".to_string(), "NZ".to_string()],
        0x28 => vec!["JP".to_string(), "Z".to_string()],
        0x30 => vec!["JP".to_string(), "NC".to_string()],
        0x38 => vec!["JP".to_string(), "C".to_string()],
        _ => return None,
    };

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, offset as u8],
        length: 2,
        mnemonic: instr.join(" ").to_string() + " " + &format!("{:04X}", target),
        verb: instr[0].clone(),
        operands: if instr[1].is_empty() {
            vec![Operand::Offset(offset)]
        } else {
            vec![Operand::Conditional(instr[1].clone()), Operand::Offset(offset)]
        }
    })
}

pub fn hl_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "JP HL".into(),
        verb: "JP".into(),
        operands: vec![Operand::Register16("HL".to_string())],
    })
}
