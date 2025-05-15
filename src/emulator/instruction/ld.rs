use crate::emulator::{bus, cpu};
use crate::emulator::bus::BusView;
use crate::emulator::cpu::CpuView;
use crate::emulator::disassemble::{Disasm, Operand};
use crate::emulator::util;

pub fn r8_n8(cpu: &mut cpu::CPU, bus: &mut bus::Bus, opcode: u8) {
    let register_code = (opcode >> 3) & 0b111;
    let register = util::get_register_by_code(register_code);
    cpu.pc += 1;
    let value = match bus.read_byte(cpu.pc) {
        Ok(byte) => byte,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    cpu.set_register(bus, register, value);
    cpu.pc += 1;
}

pub fn r16_n16(cpu: &mut cpu::CPU, bus: &mut bus::Bus, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    cpu.pc += 1;
    let value = match bus.read_word(cpu.pc) {
        Ok(word) => word,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    cpu.set_register_pair(pair, value);
    cpu.pc += 2;
}

pub fn r8_r8(cpu: &mut cpu::CPU, bus: &mut bus::Bus, opcode: u8) {
    let dst = util::get_register_by_code((opcode >> 3) & 0b111);
    let src = util::get_register_by_code(opcode & 0b111);

    if dst == src {
        cpu.pc += 1;
        return;
    }

    let value = cpu.get_register(bus, src);
    cpu.set_register(bus, dst, value);
    cpu.pc += 1;
}

pub fn addr_of_r16_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let addr = cpu.get_register_pair(pair);
    let value = cpu.get_register(bus, util::Register::A);

    match bus.write_byte(addr, value) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e)
    }

    cpu.pc +=1;
}

pub fn a_addr_of_r16(cpu: &mut cpu::CPU, bus: &mut bus::Bus, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let addr = cpu.get_register_pair(pair);

    let value = match bus.read_byte(addr) {
        Ok(byte) => byte,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    cpu.set_register(bus, util::Register::A, value);

    cpu.pc +=1;
}

pub fn addr_of_hl_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus, should_increase: bool) {
    let value = cpu.get_register(util::Register::A);

    let h = cpu.get_register(util::Register::H);
    let l = cpu.get_register(util::Register::L);
    bus.write_byte(((h as u16) << 8) | (l as u16), value).unwrap();

    let hl = cpu.get_register_pair(util::RegisterPair::HL);
    if should_increase {
        cpu.set_register_pair(util::RegisterPair::HL, hl.wrapping_add(1));
    } else {
        cpu.set_register_pair(util::RegisterPair::HL, hl.wrapping_sub(1));
    }

    cpu.pc += 1;
}

pub fn a_addr_of_hl(cpu: &mut cpu::CPU, bus: &mut bus::Bus, should_increase: bool) {
    //bus
    //                 .read_byte(((self.h as u16) << 8) | (self.l as u16))
    //                 .unwrap(),
    let h = cpu.get_register(util::Register::H);
    let l = cpu.get_register(util::Register::L);
    let value = bus.read_byte(((h as u16) << 8) | (l as u16)).unwrap();

    cpu.set_register(bus, util::Register::A, value);

    let hl = cpu.get_register_pair(util::RegisterPair::HL);
    if should_increase {
        cpu.set_register_pair(util::RegisterPair::HL, hl.wrapping_add(1));
    } else {
        cpu.set_register_pair(util::RegisterPair::HL, hl.wrapping_sub(1));
    }

    cpu.pc += 1;
}

pub fn a16_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let addr = bus.read_word(cpu.pc + 1).unwrap();
    let content = cpu.get_register(util::Register::A);

    let _ = bus.write_byte(addr, content);

    cpu.pc += 3;
}

pub fn a_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let addr = bus.read_word(cpu.pc + 1).unwrap();
    let content = bus.read_byte(addr).unwrap();

    cpu.set_register(bus, util::Register::A, content);

    cpu.pc += 3;
}

pub fn a16_sp(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let addr = bus.read_word(cpu.pc + 1).unwrap();
    let sp = cpu.get_register_pair(util::RegisterPair::SP);

    let _ = bus.write_word(addr, sp);
    cpu.pc += 3;
}

pub fn hl_sp_e8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let sp = cpu.sp;
    let offset = bus.read_byte(cpu.pc + 1).unwrap() as i8 as i16;

    let result = (sp as i16).wrapping_add(offset) as u16;
    cpu.set_register_pair(util::RegisterPair::HL, result);

    let lo_sp = sp & 0xFF;
    let lo_offset = offset as u16 & 0xFF;

    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = ((lo_sp ^ lo_offset ^ (lo_sp + lo_offset)) & 0x10) == 0x10;
    cpu.flags.carry = ((lo_sp ^ lo_offset ^ (lo_sp + lo_offset)) & 0x100) == 0x100;

    cpu.pc += 2;
}

pub fn r8_n8_disasm(bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let reg = util::get_register_by_code((opcode >> 3) & 0b111);
    let content = bus.read_byte(addr + 1).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, bus.read_byte(addr + 1).unwrap_or(0)],
        length: 2,
        mnemonic: format!("LD {}, ${:02X}", reg, content),
        verb: "LD".into(),
        operands: vec![Operand::Register8(reg.to_string()), Operand::Immediate8(content)],
    })
}

pub fn r16_n16_disasm(bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let content = bus.read_word(addr + 1).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, content as u8, (content >> 8) as u8],
        length: 3,
        mnemonic: format!("LD {}, ${:04X}", pair, content),
        verb: "LD".into(),
        operands: vec![Operand::Register16(pair.to_string()), Operand::Immediate16(content)],
    })
}

pub fn r8_r8_disasm(_bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let dst = util::get_register_by_code((opcode >> 3) & 0b111);
    let src = util::get_register_by_code(opcode & 0b111);
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("LD {}, {}", dst, src),
        verb: "LD".into(),
        operands: vec![Operand::Register8(dst.to_string()), Operand::Register8(src.to_string())],
    })
}

pub fn addr_of_r16_a_disasm(_bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("LD ({}), A", pair),
        verb: "LD".into(),
        operands: vec![Operand::MemoryIndirect(pair.to_string()), Operand::Register8("A".into())],
    })
}

pub fn a_addr_of_r16_disasm(_bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("LD A, ({})", pair),
        verb: "LD".into(),
        operands: vec![Operand::Register8("A".into()), Operand::MemoryIndirect(pair.to_string())],
    })
}

pub fn addr_of_hl_a_disasm(_bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let hl = if opcode == 0x22 {
        "HL+"
    } else {
        "HL-"
    };
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("LD ({}), A", hl),
        verb: "LD".into(),
        operands: vec![Operand::MemoryIndirect(hl.into()), Operand::Register8("A".into())],
    })
}

pub fn a_addr_of_hl_disasm(_bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let hl = if opcode == 0x2A {
        "HL+"
    } else {
        "HL-"
    };
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("LD A, ({})", hl),
        verb: "LD".into(),
        operands: vec![Operand::Register8("A".into()), Operand::MemoryIndirect(hl.into())],
    })
}

pub fn a16_a_disasm(bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let target = bus.read_word(addr).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, target as u8, (target >> 8) as u8],
        length: 3,
        mnemonic: format!("LD ({:04X}), A", target),
        verb: "LD".into(),
        operands: vec![Operand::Address(target), Operand::Register8("A".into())],
    })
}

pub fn a_a16_disasm(bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let source = bus.read_word(addr).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, source as u8, (source >> 8) as u8],
        length: 3,
        mnemonic: format!("LD A, ({:04X})", source),
        verb: "LD".into(),
        operands: vec![Operand::Register8("A".into()), Operand::Address(source)],
    })
}

pub fn a16_sp_disasm(bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let dest = bus.read_word(addr).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, dest as u8, (dest >> 8) as u8],
        length: 3,
        mnemonic: format!("LD ({:04X}), SP", dest),
        verb: "LD".into(),
        operands: vec![Operand::Address(dest), Operand::Register16("SP".into())],
    })
}

pub fn hl_sp_e8_disasm(bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let offset = bus.read_byte(addr + 1).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, offset],
        length: 2,
        mnemonic: format!("LD HL, SP+{:+}", offset as i8),
        verb: "LD".into(),
        operands: vec![Operand::Register16("HL".into()), Operand::Register16("SP".into()), Operand::Offset(offset as i8)],
    })
}
