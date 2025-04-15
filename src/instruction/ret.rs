use crate::cpu;

pub fn ret(cpu: &mut cpu::CPU, opcode: u8) {
    let addr = match cpu.bus.borrow_mut().pop_word(&mut cpu.sp) {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to pop return address: {}", e);
            cpu.pc += 1;
            return;
        }
    };

    let should_jump = match opcode >> 4 {
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump || opcode == 0o311 {
        cpu.pc = addr;
    } else {
        cpu.pc += 1;
    }
}
