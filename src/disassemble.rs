use crate::util;

pub fn disassemble(rom: &Vec<u8>, pc: u16) -> (String, u16) {
    let opcode = rom.get(pc as usize).copied().unwrap_or(0x00);
    match opcode {
        0o00 => ("NOP".into(), 1),
        0o04 | 0o14 | 0o24 | 0o34 | 0o44 | 0o54 | 0o64 | 0o74 => {
            // INC register
            let register_code = (opcode >> 3) & 0b111;
            let register = util::get_register_by_code(register_code);
            (format!("INC {}", register), 1)
        }
        0o05 | 0o15 | 0o25 | 0o35 | 0o45 | 0o55 | 0o65 | 0o75 => {
            // DEC register
            let register_code = (opcode >> 3) & 0b111;
            let register = util::get_register_by_code(register_code);
            (format!("DEC {}", register), 1)
        }
        _ => ("???".into(), 1)
    }
}

