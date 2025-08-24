use crate::disassemble;
use crate::emulator::bus::Bus;
use crate::emulator::instruction;
use crate::emulator::instruction::Instruction;
use crate::emulator::util::get_register_pair_by_code;
use crate::emulator::util::Register;
use crate::emulator::util::RegisterPair;

pub struct Flags {
    pub zero: bool,
    pub subtraction: bool,
    pub half_carry: bool,
    pub carry: bool,
}

pub struct CPU {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub flags: Flags,

    pub sp: u16,
    pub pc: u16,

    pub ie_enable_delay: bool,

    pub is_halting: bool,

    pub should_trace_log: bool,
}

impl CPU {
    pub fn new(should_trace_log: bool) -> Self {
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            flags: Flags {
                zero: true,
                subtraction: false,
                half_carry: false,
                carry: true,
            },
            sp: 0xFFFE,
            pc: 0x0100,
            ie_enable_delay: false,
            is_halting: false,
            should_trace_log,
        }
    }

    pub fn reset(&mut self) {
        // TODO: reset bus as well
        *self = CPU::new(self.should_trace_log);
    }

    fn decode(&mut self, opcode: u8) -> Instruction {
        match opcode {
            0x00 => {
                self.pc += 1;
            }
            0o363 => {
                instruction::di::di(self, bus);
            }
            0o373 => {
                instruction::ei::ei(self);
            }
            0o166 | 0o20 => {
                instruction::halt::halt(self);
            }
            0o03 | 0o13 | 0o23 | 0o33 | 0o43 | 0o53 | 0o63 | 0o73 => {
                let pair = get_register_pair_by_code(opcode >> 4);
                if (opcode >> 3) & 0 == 1 {
                    instruction::inc::r16(self, pair);
                } else {
                    instruction::dec::r16(self, pair);
                }
            }
            0o04 | 0o14 | 0o24 | 0o34 | 0o44 | 0o54 | 0o64 | 0o74 => {
                instruction::inc::r8(self, bus, opcode);
            }
            0o05 | 0o15 | 0o25 | 0o35 | 0o45 | 0o55 | 0o65 | 0o75 => {
                instruction::dec::r8(self, bus, opcode);
            }
            0o06 | 0o16 | 0o26 | 0o36 | 0o46 | 0o56 | 0o66 | 0o76 => {
                instruction::ld::r8_n8(self, bus, opcode);
            }
            0o100..=0o175 | 0o167..=0o177 => {
                instruction::ld::r8_r8(self, bus, opcode);
            }
            0o01 | 0o21 | 0o41 | 0o61 => {
                instruction::ld::r16_n16(self, bus, opcode);
            }
            0o02 | 0o22 => {
                instruction::ld::addr_of_r16_a(self, bus, opcode);
            }
            0o12 | 0o32 => {
                instruction::ld::a_addr_of_r16(self, bus, opcode);
            }
            0o42 => instruction::ld::addr_of_hl_a(self, bus, true),
            0o52 => instruction::ld::a_addr_of_hl(self, bus, true),
            0o62 => instruction::ld::addr_of_hl_a(self, bus, false),
            0o72 => instruction::ld::a_addr_of_hl(self, bus, false),
            0o352 => instruction::ld::a16_a(self, bus),
            0o372 => instruction::ld::a_a16(self, bus),
            0o10 => instruction::ld::a16_sp(self, bus),
            0o370 => instruction::ld::hl_sp_e8(self, bus),
            0o340 => instruction::ldh::a8_a(self, bus),
            0o360 => instruction::ldh::a_a8(self, bus),
            0o342 => instruction::ldh::c_a(self, bus),
            0o362 => instruction::ldh::a_c(self, bus),
            0o301 | 0o321 | 0o341 | 0o361 => {
                instruction::pop::r16(self, bus, opcode);
            }
            0o305 | 0o325 | 0o345 | 0o365 => {
                instruction::push::r16(self, bus, opcode);
            }
            0o200..=0o207 => {
                instruction::add::r8(self, bus, opcode);
            }
            0o11 | 0o31 | 0o51 | 0o71 => {
                instruction::add::r16(self, opcode);
            }
            0o350 => instruction::add::sp_e8(self, bus),
            0o306 => instruction::add::a_n8(self, bus),
            0o210..=0o217 => {
                instruction::adc::r8(self, bus, opcode);
            }
            0o316 => instruction::adc::a_n8(self, bus),
            0o220..=0o227 => {
                instruction::sub::r8(self, bus, opcode);
            }
            0o326 => instruction::sub::a_n8(self, bus),
            0o230..=0o237 => {
                instruction::sbc::r8(self, bus, opcode);
            }
            0o336 => instruction::sbc::a_n8(self, bus),
            0o240..=0o247 => {
                instruction::and::r8(self, bus, opcode);
            }
            0o346 => instruction::and::a_n8(self, bus),
            0o250..=0o257 => {
                instruction::xor::r8(self, bus, opcode);
            }
            0o356 => instruction::xor::a_n8(self, bus),
            0o260..=0o267 => {
                instruction::or::r8(self, bus, opcode);
            }
            0o366 => instruction::or::a_n8(self, bus),
            0o270..=0o277 => {
                instruction::cp::r8(self, bus, opcode);
            }
            0o07 => instruction::rotate::rlca(self, bus),
            0o17 => instruction::rotate::rrca(self, bus),
            0o27 => instruction::rotate::rla(self, bus),
            0o37 => instruction::rotate::rra(self, bus),
            0o47 => instruction::accumulator::daa(self, bus),
            0o57 => instruction::accumulator::cpl(self, bus),
            0o67 => instruction::carry::scf(self),
            0o77 => instruction::carry::ccf(self),
            0o376 => instruction::cp::a_n8(self, bus),
            0o30 | 0o40 | 0o50 | 0o60 | 0o70 => {
                instruction::jump::e8(self, bus, opcode);
            }
            0o351 => instruction::jump::hl(self),
            0o302 | 0o303 | 0o312 | 0o322 | 0o332 => {
                instruction::jump::a16(self, bus, opcode);
            }
            0o300 | 0o310 | 0o311 | 0o320 | 0o330 | 0o331 => {
                instruction::ret::ret(self, bus, opcode);
            }
            0o304 | 0o314 | 0o315 | 0o324 | 0o334 => {
                instruction::call::call(self, bus, opcode);
            }
            0o307 | 0o317 | 0o327 | 0o337 | 0o347 | 0o357 | 0o367 | 0o377 => {
                instruction::rst::rst(self, bus, opcode);
            }
            0o323 | 0o333 | 0o343 | 0o353 | 0o344 | 0o354 | 0o364 | 0o374 | 0o335 | 0o355
            | 0o375 => {
                return;
            }
            _ => {
                unimplemented!("Opcode {:02X} not implemented yet", opcode);
            }
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        if self.ie_enable_delay {
            self.ie_enable_delay = false;
            bus.interrupts.ime = true;
        }

        if self.is_halting {
            return;
        }

        self.handle_interrupts(bus);

        // fetch
        let opcode = match bus.read_byte(self.pc) {
            Ok(byte) => byte,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        let instruction = self.decode(opcode);
        

        if self.should_trace_log {
            if let Some(disasm) = disassemble(&*bus, self.pc) {
                println!(
                    "{:04X}: {:<12} | A:{:02X} F:{}{}{}{} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X}",
                    disasm.address,
                    disasm.mnemonic,
                    self.a,
                    if self.flags.zero { "Z" } else { "-" },
                    if self.flags.subtraction { "N" } else { "-" },
                    if self.flags.half_carry { "H" } else { "-" },
                    if self.flags.carry { "C" } else { "-" },
                    self.b, self.c, self.d, self.e, self.h, self.l,
                    self.sp,
                );
            } else {
                println!("{:04X}: <undisassembled>", self.pc);
            }
        }
    }

    pub fn get_register(&self, bus: &mut Bus, register: Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::HL => bus
                .read_byte(((self.h as u16) << 8) | (self.l as u16))
                .unwrap(),
        }
    }

    pub fn get_flags_as_byte(&self) -> u8 {
        (self.flags.zero as u8) << 7
            | (self.flags.subtraction as u8) << 6
            | (self.flags.half_carry as u8) << 5
            | (self.flags.carry as u8) << 4
    }

    pub fn set_flags_as_byte(&mut self, content: u8) {
        self.flags.zero = content & 0b1000_0000 != 0;
        self.flags.subtraction = content & 0b0100_0000 != 0;
        self.flags.half_carry = content & 0b0010_0000 != 0;
        self.flags.carry = content & 0b0001_0000 != 0;
    }

    pub fn set_register(&mut self, bus: &mut Bus, register: Register, val: u8) {
        match register {
            Register::A => self.a = val,
            Register::B => self.b = val,
            Register::C => self.c = val,
            Register::D => self.d = val,
            Register::E => self.e = val,
            Register::H => self.h = val,
            Register::L => self.l = val,
            Register::HL => bus
                .write_byte(((self.h as u16) << 8) | (self.l as u16), val)
                .unwrap(),
        }
    }

    pub fn get_register_pair(&mut self, pair: RegisterPair) -> u16 {
        match pair {
            RegisterPair::BC => ((self.b as u16) << 8) | (self.c as u16),
            RegisterPair::DE => ((self.d as u16) << 8) | (self.e as u16),
            RegisterPair::HL => ((self.h as u16) << 8) | (self.l as u16),
            RegisterPair::SP => self.sp,
        }
    }

    pub fn set_register_pair(&mut self, pair: RegisterPair, val: u16) {
        match pair {
            RegisterPair::BC => {
                self.b = (val >> 8) as u8;
                self.c = val as u8;
            }
            RegisterPair::DE => {
                self.d = (val >> 8) as u8;
                self.e = val as u8;
            }
            RegisterPair::HL => {
                self.h = (val >> 8) as u8;
                self.l = val as u8;
            }
            RegisterPair::SP => {
                self.sp = val;
            }
        }
    }

    fn handle_interrupts(&mut self, bus: &mut Bus) {
        let interrupts = &mut bus.interrupts;
        if !interrupts.ime {
            return;
        };

        for (idx, interrupt) in interrupts.registers.iter_mut().enumerate() {
            if interrupt.is_enabled && interrupt.is_requested {
                interrupts.ime = false;
                interrupt.is_requested = false;

                // 2 NOP

                if let Err(e) = bus.push_word(&mut self.sp, self.pc) {
                    eprintln!("Failed to push PC during interrupt: {}", e);
                    return;
                }

                self.pc = idx as u16 + 0x40;
                break;
            }
        }
    }
}
