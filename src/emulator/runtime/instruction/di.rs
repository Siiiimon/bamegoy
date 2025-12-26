use crate::emulator::runtime::bus;
use crate::emulator::runtime::{cpu, disassemble::Disasm};

pub fn di(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    bus.interrupts.ime = false;
    cpu.pc += 1;
}

pub fn di_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "DI".into(),
        verb: "DI".into(),
        operands: vec![],
    })
}
