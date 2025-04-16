use crate::{cpu, disassemble::Disasm};

pub fn rst(cpu: &mut cpu::CPU, opcode: u8) {
    let addr = ((opcode >> 3) & 0b111) * 8;

    let _ = cpu.bus.borrow_mut().push_word(&mut cpu.sp, cpu.pc + 1);
    cpu.pc = addr as u16;
}

pub fn rst_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    let target = ((opcode >> 3) & 0b111) * 8;

    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("RST ${:02X}", target),
    })
}
