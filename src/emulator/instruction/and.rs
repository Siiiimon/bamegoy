use crate::emulator::bus;
use crate::emulator::disassemble::Operand;
use crate::emulator::{cpu, disassemble::Disasm, util};

pub fn r8(cpu: &mut cpu::CPU, opcode: u8) {
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(util::Register::A);
    let x = cpu.get_register(register);

    let result = x & a;
    cpu.set_register(util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = true;

    cpu.pc += 1;
}

pub fn a_n8(cpu: &mut cpu::CPU) {
    let a = cpu.get_register(util::Register::A);
    let x = cpu.bus.borrow().read_byte(cpu.pc + 1).unwrap();

    let result = x & a;
    cpu.set_register(util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = true;

    cpu.pc += 2;
}

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

