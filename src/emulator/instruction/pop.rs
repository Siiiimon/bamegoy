use crate::emulator::bus;
use crate::emulator::disassemble::Operand;
use crate::emulator::{cpu, disassemble::Disasm, util};



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
