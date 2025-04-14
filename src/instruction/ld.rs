use crate::cpu;
use crate::util;

pub fn r8_n8(cpu: &mut cpu::CPU, opcode: u8) {
    let register_code = (opcode >> 3) & 0b111;
    let register = util::get_register_by_code(register_code);
    cpu.pc += 1;
    let value = match cpu.bus.borrow().rom_read_byte(cpu.pc) {
        Some(byte) => byte,
        None => {
            eprintln!("Tried to read invalid ROM address: {:04X}", cpu.pc);
            return;
        }
    };
    cpu.set_register(register, value);
    cpu.pc += 1;
}

pub fn r16_n16(cpu: &mut cpu::CPU, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    cpu.pc += 1;
    let value = match cpu.bus.borrow().rom_read_word(cpu.pc) {
        Some(word) => word,
        None => {
            eprintln!("Tried to read invalid ROM address: {:04X}", cpu.pc);
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
