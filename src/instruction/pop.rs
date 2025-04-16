use crate::{cpu, disassemble::Disasm, util};

pub fn r16(cpu: &mut cpu::CPU, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let content = cpu.bus.borrow_mut().pop_word(&mut cpu.sp).unwrap();

    if pair == util::RegisterPair::SP {
        cpu.set_register(util::Register::A, (content >> 8) as u8);
        cpu.set_flags_as_byte(content as u8);
    } else {
        cpu.set_register_pair(pair, content);
    };

    cpu.pc += 1;
}

pub fn r16_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);

    Some(Disasm{
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: format!("POP {}", pair)
    })
}
