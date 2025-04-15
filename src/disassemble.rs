use crate::util;

pub fn disassemble(rom: &Vec<u8>, mut pc: u16) -> (String, u16) {
    let opcode = rom.get(pc as usize).copied().unwrap();
    match opcode {
        0o00 => ("NOP".into(), 1),
        0o363 => {
            ("DI".into(), 1)
        }
        0o373 => {
            ("EI".into(), 1)
        }
        0o166 => {
            ("HALT".into(), 1)
        }
        0o03 | 0o13 | 0o23 | 0o33 | 0o43 | 0o53 | 0o63 | 0o73 => {
            let pair = util::get_register_pair_by_code(opcode >> 4);
            if (opcode >> 3) & 0 == 1 {
                (format!("INC {}", pair), 1)
            } else {
                (format!("DEC {}", pair), 1)
            }
        }
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
            (format!("LD {} {}", register, *value), 2)
        }
        0o100..=0o175 | 0o167..=0o177 => {
            let dst = util::get_register_by_code((opcode >> 3) & 0b111);
            let src = util::get_register_by_code(opcode & 0b111);
            (format!("LD {} {}", dst, src), 1)
        }
        0o01 | 0o21 | 0o41 | 0o61 => {
            let pair = util::get_register_pair_by_code(opcode >> 4);
            let hi = match rom.get((pc as usize) + 1) {
                Some(byte) => byte,
                None => {
                    eprintln!("Tried to read invalid ROM address: {:04X}", pc);
                    return ("???".into(), 3);
                }
            };
            let lo = match rom.get((pc as usize) + 2) {
                Some(byte) => byte,
                None => {
                    eprintln!("Tried to read invalid ROM address: {:04X}", pc);
                    return ("???".into(), 3);
                }
            };
            (format!("LD {} {}", pair, ((*hi as u16) << 8) | (*lo as u16)), 3)

        }
        0o02 | 0o22 => {
            let pair = util::get_register_pair_by_code(opcode >> 4);
            (format!("LD [{}] A", pair), 1)
        }
        0o12 | 0o32 => {
            let pair = util::get_register_pair_by_code(opcode >> 4);
            (format!("LD A [{}]", pair), 1)
        }
        0o42 => (format!("LD [HL+] A"), 1),
        0o52 => (format!("LD A [HL+]"), 1),
        0o62 => (format!("LD [HL-] A"), 1),
        0o72 => (format!("LD A [HL-]"), 1),
        0o301 | 0o321 | 0o341 | 0o361 => {
            let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
            if pair == util::RegisterPair::SP {
                ("POP AF".into(), 1)
            } else {
                (format!("POP {}", pair), 1)
            }
        }
        0o305 | 0o325 | 0o345 | 0o365 => {
            if (opcode >> 4) & 0b11 == 0b11 {
                ("PUSH AF".into(), 1)
            } else {
                (format!("PUSH {}", util::get_register_pair_by_code((opcode >> 4) & 0b11)), 1)
            }
        }
        0o200..=0o207 => {
            let register = util::get_register_by_code(opcode & 0b111);
            (format!("ADD A {}", register), 1)
        }
        0o11 | 0o31 | 0o51 | 0o71 => {
            (format!("ADD HL {}", util::get_register_pair_by_code((opcode >> 4) & 0b11)), 1)
        }
        0o210..=0o217 => {
            let register = util::get_register_by_code(opcode & 0b111);
            (format!("ADC A {}", register), 1)
        }
        0o220..=0o227 => {
            let register = util::get_register_by_code(opcode & 0b111);
            (format!("SUB A {}", register), 1)
        }
        0o230..=0o237 => {
            let register = util::get_register_by_code(opcode & 0b111);
            (format!("SBC A {}", register), 1)
        }
        0o240..=0o247 => {
            let register = util::get_register_by_code(opcode & 0b111);
            (format!("AND A {}", register), 1)
        }
        0o250..=0o257 => {
            let register = util::get_register_by_code(opcode & 0b111);
            (format!("XOR A {}", register), 1)
        }
        0o260..=0o267 => {
            let register = util::get_register_by_code(opcode & 0b111);
            (format!("OR A {}", register), 1)
        }
        0o270..=0o277 => {
            let register = util::get_register_by_code(opcode & 0b111);
            (format!("CP A {}", register), 1)
        }
        0o30 | 0o40 | 0o50 | 0o60 | 0o70 => {
            let offset = match rom.get((pc as usize) + 1) {
                Some(byte) => *byte as i8,
                None => {
                    eprintln!("Tried to read invalid ROM address: {:04X}", pc);
                    return ("???".into(), 2);
                }
            };
            let conditional = match opcode >> 4 {
                1 => "",
                2 => "NZ ",
                3 => "Z ",
                4 => "NC ",
                5 => "C ",
                _ => "??? ",
            };
            (format!("JR {}{}", conditional, offset as u8), 2)
        }
        0o351 => (format!("JP HL"), 2),
        0o302 | 0o303 | 0o312 | 0o322 | 0o332  => {
            let lo = rom.get((pc + 1) as usize).unwrap();
            let hi = rom.get((pc + 2) as usize).unwrap();

            let conditional = match opcode >> 4 {
                2 => "NZ",
                3 => "Z",
                4 => "NC",
                5 => "C",
                _ => "???",
            };

            if opcode == 0o303 {
                (format!("JP {:04X}", ((*hi as u16) << 8) | *lo as u16), 3)
            } else {
                (format!("JP {} {:04X}", conditional, ((*hi as u16) << 8) | *lo as u16), 3)
            }
        }
        0o300 | 0o310 | 0o311 | 0o320 | 0o330 => {
            let conditional = match opcode >> 4 {
                2 => "NZ",
                3 => "Z",
                4 => "NC",
                5 => "C",
                _ => "???",
            };

            if opcode == 0o311 {
                ("RET".into(), 1)
            } else {
                (format!("RET {}", conditional), 1)
            }
        }
        0o331 => ("RETI".into(), 1),
        0o304 | 0o314 | 0o315 | 0o324 | 0o334 => {
            let conditional = match opcode >> 4 {
                2 => "NZ",
                3 => "Z",
                4 => "NC",
                5 => "C",
                _ => "???",
            };

            if opcode == 0o315 {
                ("CALL".into(), 3)
            } else {
                (format!("CALL {}", conditional), 3)
            }
        }
        _ => {
            (format!("UNKNOWN: 0o{:03o}", opcode), 1)
        }
    }
}

