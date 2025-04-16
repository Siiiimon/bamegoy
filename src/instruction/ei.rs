use crate::{cpu, disassemble::Disasm};

pub fn ei(cpu: &mut cpu::CPU) {
    cpu.ie_enable_delay = true;
    cpu.pc += 1;
}

pub fn ei_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "EI".into(),
    })
}
