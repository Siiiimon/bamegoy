use crate::emulator::bus;
use crate::emulator::disassemble::Operand;
use crate::emulator::{cpu, disassemble::Disasm, util};

pub fn r16(cpu: &mut cpu::CPU, bus: &mut bus::Bus, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let content = bus.pop_word(&mut cpu.sp).unwrap();

    if pair == util::RegisterPair::SP {
        cpu.set_register(bus, util::Register::A, (content >> 8) as u8);
        cpu.set_flags_as_byte(content as u8);
    } else {
        cpu.set_register_pair(pair, content);
    };

    cpu.pc += 1;
}

pub fn r16_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);

    Some(Disasm{
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("POP {}", pair),
        verb: "POP".into(),
        operands: vec![Operand::Register16(pair.to_string())],
    })
}
