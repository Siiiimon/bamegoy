use crate::util;

pub fn disassemble(rom: &Vec<u8>, mut pc: u16) -> (String, u16) {
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
        0o06 | 0o16 | 0o26 | 0o36 | 0o46 | 0o56 | 0o66 | 0o76 => {
            // LD register
            let register_code = (opcode >> 3) & 0b111;
            let register = util::get_register_by_code(register_code);
            pc += 1;
            let value = match rom.get(pc as usize) {
                Some(byte) => byte,
                None => {
                    eprintln!("Tried to read invalid ROM address: {:04X}", pc);
                    return ("???".into(), 2);
                }
            };
            (format!("LD {} {}", register, value), 2)
        }
        0o303 => {
            // JP a16
            let lo = rom.get((pc + 1) as usize).unwrap();
            let hi = rom.get((pc + 2) as usize).unwrap();
            (format!("JP {:04X}", ((*hi as u16) << 8) | *lo as u16), 3)
        }
        _ => ("???".into(), 1)
    }
}

