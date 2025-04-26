use crate::emulator::bus;
use crate::emulator::disassemble::Operand;
use crate::emulator::{cpu, disassemble::Disasm};

pub fn rst(cpu: &mut cpu::CPU, bus: &mut bus::Bus, opcode: u8) {
    let addr = ((opcode >> 3) & 0b111) * 8;

    let _ = bus.push_word(&mut cpu.sp, cpu.pc + 1);
    cpu.pc = addr as u16;
}

pub fn rst_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let target = ((opcode >> 3) & 0b111) * 8;

    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("RST ${:02X}", target),
        verb: "RST".into(),
        operands: vec![Operand::Address(target as u16)],
    })
}
