use crate::{cpu, util};

pub fn r16(cpu: &mut cpu::CPU, opcode: u8) {
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);

    let content = if pair == util::RegisterPair::SP {
        let hi = cpu.get_register(util::Register::A);
        let lo = cpu.get_flags_as_byte();
        ((hi << 8) | lo) as u16
    } else {
        cpu.get_register_pair(pair)
    };

    let _ = cpu.bus.borrow_mut().push_word(&mut cpu.sp, content);
    cpu.pc += 1;
}
