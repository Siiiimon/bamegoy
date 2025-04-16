use crate::cpu;
use crate::util;

pub fn r8_n8(cpu: &mut cpu::CPU, opcode: u8) {
    let register_code = (opcode >> 3) & 0b111;
    let register = util::get_register_by_code(register_code);
    cpu.pc += 1;
    let value = match cpu.bus.borrow().read_byte(cpu.pc) {
        Ok(byte) => byte,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    cpu.set_register(register, value);
    cpu.pc += 1;
}

pub fn r16_n16(cpu: &mut cpu::CPU, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    cpu.pc += 1;
    let value = match cpu.bus.borrow().read_word(cpu.pc) {
        Ok(word) => word,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    cpu.set_register_pair(pair, value);
    cpu.pc += 2;
}

pub fn r8_r8(cpu: &mut cpu::CPU, opcode: u8) {
    let dst = util::get_register_by_code((opcode >> 3) & 0b111);
    let src = util::get_register_by_code(opcode & 0b111);

    if dst == src {
        cpu.pc += 1;
        return;
    }

    cpu.set_register(dst, cpu.get_register(src));
    cpu.pc += 1;
}

pub fn addr_of_r16_a(cpu: &mut cpu::CPU, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let addr = cpu.get_register_pair(pair);
    let value = cpu.get_register(util::Register::A);

    match cpu.bus.borrow_mut().write_byte(addr, value) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e)
    }

    cpu.pc +=1;
}

pub fn a_addr_of_r16(cpu: &mut cpu::CPU, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let addr = cpu.get_register_pair(pair);

    let value = match cpu.bus.borrow().read_byte(addr) {
        Ok(byte) => byte,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    cpu.set_register(util::Register::A, value);

    cpu.pc +=1;
}

pub fn addr_of_hl_a(cpu: &mut cpu::CPU, should_increase: bool) {
    let value = cpu.get_register(util::Register::A);
    cpu.set_register(util::Register::HL, value);

    let hl = cpu.get_register_pair(util::RegisterPair::HL);
    if should_increase {
        cpu.set_register_pair(util::RegisterPair::HL, hl.wrapping_add(1));
    } else {
        cpu.set_register_pair(util::RegisterPair::HL, hl.wrapping_sub(1));
    }

    cpu.pc += 1;
}

pub fn a_addr_of_hl(cpu: &mut cpu::CPU, should_increase: bool) {
    let value = cpu.get_register(util::Register::HL);
    cpu.set_register(util::Register::A, value);

    let hl = cpu.get_register_pair(util::RegisterPair::HL);
    if should_increase {
        cpu.set_register_pair(util::RegisterPair::HL, hl.wrapping_add(1));
    } else {
        cpu.set_register_pair(util::RegisterPair::HL, hl.wrapping_sub(1));
    }

    cpu.pc += 1;
}

pub fn a16_a(cpu: &mut cpu::CPU) {
    let addr = cpu.bus.borrow().read_word(cpu.pc + 1).unwrap();
    let content = cpu.get_register(util::Register::A);

    let _ = cpu.bus.borrow_mut().write_byte(addr, content);

    cpu.pc += 3;
}

pub fn a_a16(cpu: &mut cpu::CPU) {
    let addr = cpu.bus.borrow().read_word(cpu.pc + 1).unwrap();
    let content = cpu.bus.borrow().read_byte(addr).unwrap();

    cpu.set_register(util::Register::A, content);

    cpu.pc += 3;
}

pub fn a16_sp(cpu: &mut cpu::CPU) {
    let addr = cpu.bus.borrow().read_word(cpu.pc + 1).unwrap();
    let sp = cpu.get_register_pair(util::RegisterPair::SP);

    let _ = cpu.bus.borrow_mut().write_word(addr, sp);
    cpu.pc += 3;
}

pub fn hl_sp_e8(cpu: &mut cpu::CPU) {
    let sp = cpu.sp;
    let offset = cpu.bus.borrow().read_byte(cpu.pc + 1).unwrap() as i8 as i16;

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

