use crate::emulator::runtime::bus;
use crate::emulator::runtime::disassemble::Operand;
use crate::emulator::runtime::{cpu, disassemble::Disasm};
use crate::emulator::util;

pub fn r8(cpu: &mut cpu::CPU, bus: &mut bus::Bus, opcode: u8) {
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(bus, util::Register::A);
    let x = cpu.get_register(bus, register);

    let temp = x.wrapping_add(a);
    let value = if cpu.flags.carry { temp + 1 } else { temp };
    cpu.set_register(bus, util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a || value < temp;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) + (cpu.flags.carry as u8) > 0x0F;

    cpu.pc += 1;
}

pub fn a_n8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let a = cpu.get_register(bus, util::Register::A);
    let x = bus.read_byte(cpu.pc + 1).unwrap();

    let temp = a.wrapping_add(x);
    let value = if cpu.flags.carry { temp + 1 } else { temp };
    cpu.set_register(bus, util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a || value < temp;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) + (cpu.flags.carry as u8) > 0x0F;

    cpu.pc += 2;
}

pub fn r8_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let register = util::get_register_by_code(opcode & 0b111);

    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("ADC A, {}", register),
        verb: "ADC".into(),
        operands: vec![
            Operand::Register8("A".into()),
            Operand::Register8(register.to_string()),
        ],
    })
}

pub fn a_n8_disasm(bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let imm = bus.read_byte(addr + 1).unwrap();

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, imm],
        length: 2,
        mnemonic: format!("ADC A, ${:02X}", imm),
        verb: "ADC".into(),
        operands: vec![Operand::Register8("A".into()), Operand::Immediate8(imm)],
    })
}
