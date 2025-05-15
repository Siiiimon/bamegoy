use crate::emulator::bus;
use crate::emulator::disassemble::Operand;
use crate::emulator::{cpu, disassemble::Disasm, util};
use crate::emulator::bus::BusView;
use crate::emulator::cpu::CpuView;

pub fn r8(cpu: &mut cpu::CPU, bus: &mut bus::Bus, opcode: u8) {
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(bus, util::Register::A);
    let x = cpu.get_register(bus, register);

    let result = x | a;
    cpu.set_register(bus, util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = false;

    cpu.pc += 1;
}

pub fn a_n8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let a = cpu.get_register(bus, util::Register::A);
    let x = bus.read_byte(cpu.pc + 1).unwrap();

    let result = x | a;
    cpu.set_register(bus, util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = false;

    cpu.pc += 2;
}

pub fn r8_disasm(_bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let register = util::get_register_by_code(opcode & 0b111);

    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("OR {}", register),
        verb: "OR".into(),
        operands: vec![Operand::Register8(register.to_string())],
    })
}


pub fn a_n8_disasm(bus: Box<dyn BusView>, addr: u16, opcode: u8) -> Option<Disasm> {
    let imm = bus.read_byte(addr + 1).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, imm],
        length: 2,
        mnemonic: format!("OR ${:02X}", imm),
        verb: "OR".into(),
        operands: vec![Operand::Register8("A".into()), Operand::Immediate8(imm)],
    })
}
