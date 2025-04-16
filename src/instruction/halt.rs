use crate::{cpu, disassemble::Disasm};

pub fn halt(cpu: &mut cpu::CPU) {
    cpu.is_halting = true;
}

pub(crate) fn halt_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "HALT".into(),
    })
}
