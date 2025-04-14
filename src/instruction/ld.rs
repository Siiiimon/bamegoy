use crate::util;
use crate::cpu;

pub fn r8(cpu: &mut cpu::CPU, opcode: u8) {
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
