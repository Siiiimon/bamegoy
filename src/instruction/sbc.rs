use crate::{cpu, disassemble::Disasm, util};

pub fn r8(cpu: &mut cpu::CPU, opcode: u8) {
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(util::Register::A);
    let x = cpu.get_register(register);
    let carry_in = if cpu.flags.carry { 1 } else { 0 };

    let (temp, carry1) = a.overflowing_sub(x);
    let (value, carry2) = temp.overflowing_sub(carry_in);

    cpu.set_register(util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = true;
    cpu.flags.half_carry = ((a & 0x0F).wrapping_sub((x & 0x0F) + carry_in)) & 0x10 != 0;
    cpu.flags.carry = carry1 || carry2;

    cpu.pc += 1;
}

pub fn a_n8(cpu: &mut cpu::CPU) {
    let a = cpu.get_register(util::Register::A);
    let x = cpu.bus.borrow().read_byte(cpu.pc + 1).unwrap();

    let carry = cpu.flags.carry as u8;

    let value = a.wrapping_sub(x).wrapping_sub(carry);
    cpu.set_register(util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = true;
    cpu.flags.half_carry = (a & 0x0F) < ((x & 0x0F) + carry);
    cpu.flags.carry = (a as u16) < (x as u16 + carry as u16);

    cpu.pc += 2;
}

pub fn r8_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    let register = util::get_register_by_code(opcode & 0b111);

    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("SBC A, {}", register),
    })
}

pub fn a_n8_disasm(mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    let imm = *mem.get(addr as usize + 1)?;

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, imm],
        length: 2,
        mnemonic: format!("SBC A, ${:02X}", imm),
    })
}
