use crate::cpu;

pub fn rst(cpu: &mut cpu::CPU, opcode: u8) {
    let addr = ((opcode >> 3) & 0b111) * 8;

    let _ = cpu.bus.borrow_mut().push_word(&mut cpu.sp, cpu.pc + 1);
    cpu.pc = addr as u16;
}
