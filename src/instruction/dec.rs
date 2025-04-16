use crate::{cpu, disassemble::Disasm, util};

pub fn r8(cpu: &mut cpu::CPU, opcode: u8) {
    let register_code = (opcode >> 3) & 0b111;
    let register = util::get_register_by_code(register_code);

    let current = cpu.get_register(register);
    let new = current.wrapping_sub(1);

    cpu.flags.zero = new == 0;
    cpu.flags.subtraction = true;
    cpu.flags.half_carry = (current & 0x0F) == 0;
    cpu.flags.carry = current == u8::MAX;

    cpu.set_register(register, new);

    cpu.pc += 1;
}

pub fn r16(cpu: &mut cpu::CPU, pair: util::RegisterPair) {
    let current = cpu.get_register_pair(pair);
    let new = current.wrapping_sub(1);

    cpu.set_register_pair(pair, new);

    cpu.pc += 1;
}

pub fn r16_disasm(_mem: &[u8], addr: u16, opcode: u8, pair: util::RegisterPair) -> Option<Disasm> {
    Some(Disasm{
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("DEC {}", pair)
    })
}

pub fn r8_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<crate::disassemble::Disasm> {
    let register_code = (opcode >> 3) & 0b111;
    let register = util::get_register_by_code(register_code);

    Some(Disasm{
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("DEC {}", register),
    })
}
