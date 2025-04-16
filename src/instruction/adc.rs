use crate::{cpu, disassemble::Disasm, util};

pub fn r8(cpu: &mut cpu::CPU, opcode: u8) {
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(util::Register::A);
    let x = cpu.get_register(register);

    let temp = x.wrapping_add(a);
    let value = if cpu.flags.carry {
        temp + 1
    } else {
        temp
    };
    cpu.set_register(util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a || value < temp;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) + (cpu.flags.carry as u8) > 0x0F;

    cpu.pc += 1;
}

pub fn a_n8(cpu: &mut cpu::CPU) {
    let a = cpu.get_register(util::Register::A);
    let x = cpu.bus.borrow().read_byte(cpu.pc + 1).unwrap();

    let temp = a.wrapping_add(x);
    let value = if cpu.flags.carry {
        temp + 1
    } else {
        temp
    };
    cpu.set_register(util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a || value < temp;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) + (cpu.flags.carry as u8) > 0x0F;

    cpu.pc += 2;
}

pub fn r8_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    let register = util::get_register_by_code(opcode & 0b111);

    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("ADC A, {}", register),
    })
}


pub fn a_n8_disasm(mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    let imm = *mem.get(addr as usize + 1)?;

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, imm],
        length: 2,
        mnemonic: format!("ADC A, ${:02X}", imm),
    })
}

