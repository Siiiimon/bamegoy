use crate::cpu;

pub fn call(cpu: &mut cpu::CPU, opcode: u8) {
    let lo = cpu.bus.borrow().read_byte(cpu.pc + 1).unwrap();
    let hi = cpu.bus.borrow().read_byte(cpu.pc + 2).unwrap();

    let should_jump = match opcode >> 4 {
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump || opcode == 0o315 {
        let _ = cpu.bus.borrow_mut().push_word(&mut cpu.sp, cpu.pc + 3);
        cpu.pc = ((hi as u16) << 8) | lo as u16;
    } else {
        cpu.pc += 3;
    }
}
