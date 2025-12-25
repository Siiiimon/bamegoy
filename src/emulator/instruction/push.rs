use crate::emulator::bus;
use crate::emulator::disassemble::Operand;
use crate::emulator::{cpu, disassemble::Disasm, util};

pub fn r16(cpu: &mut cpu::CPU, bus: &mut bus::Bus, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);

    let content = if pair == util::RegisterPair::SP {
        let hi = cpu.get_register(bus, util::Register::A);
        let lo = cpu.get_flags_as_byte();
        ((hi as u16) << 8) | lo as u16
    } else {
        cpu.get_register_pair(pair)
    };

    let _ = bus.push_word(&mut cpu.sp, content);
    cpu.pc += 1;
}

pub fn r16_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);

    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("PUSH {}", pair),
        verb: "PUSH".into(),
        operands: vec![Operand::Register16(pair.to_string())],
    })
}
