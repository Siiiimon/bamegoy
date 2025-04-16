use crate::bus;
use crate::{cpu, disassemble::Disasm, util};

pub fn r8(cpu: &mut cpu::CPU, opcode: u8) {
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(util::Register::A);
    let x = cpu.get_register(register);

    let value = x.wrapping_add(a);
    cpu.set_register(util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) > 0x0F;

    cpu.pc += 1;
}

pub fn a_n8(cpu: &mut cpu::CPU) {
    let a = cpu.get_register(util::Register::A);
    let x = cpu.bus.borrow().read_byte(cpu.pc + 1).unwrap();

    let value = a.wrapping_add(x);
    cpu.set_register(util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) > 0x0F;

    cpu.pc += 2;
}

pub fn r16(cpu: &mut cpu::CPU, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let hl = cpu.get_register_pair(util::RegisterPair::HL);
    let xy = cpu.get_register_pair(pair);

    let (value, carry) = hl.overflowing_add(xy);
    cpu.set_register_pair(util::RegisterPair::HL, value);

    cpu.flags.subtraction = false;
    cpu.flags.carry = carry;
    cpu.flags.half_carry = ((hl & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF;

    cpu.pc += 1;
}

pub fn sp_e8(cpu: &mut cpu::CPU) {
    let offset = cpu.bus.borrow().read_byte(cpu.pc + 1).unwrap() as i16;
    let sp = cpu.sp as i16;
    cpu.sp = sp.wrapping_add(offset) as u16;

    let lo_sp = cpu.sp as u8;
    let lo_offset = offset as u8;

    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = ((lo_sp & 0x0F) + (lo_offset & 0x0F)) > 0x0F;
    cpu.flags.carry = ((lo_sp as u16) + (lo_offset as u16)) > 0xFF;

    cpu.pc += 2;
}

pub fn r8_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let register = util::get_register_by_code(opcode & 0b111);

    Some(Disasm{
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("ADD {}", register)
    })
}

pub fn a_n8_disasm(bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let imm = bus.read_byte(addr + 1).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, imm],
        length: 2,
        mnemonic: format!("ADD A, ${:02X}", imm),
    })
}

pub fn sp_e8_disasm(bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let offset = bus.read_byte(addr + 1).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, offset as u8],
        length: 2,
        mnemonic: format!("ADD SP, {:+}", offset),
    })
}

pub fn r16_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);

    Some(Disasm{
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("ADD {}", pair)
    })
}
