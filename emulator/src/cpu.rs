// use crate::disassemble;
use crate::bus::Bus;
use crate::instruction;
use crate::instruction::Instruction;
use crate::util::Register;
use crate::util::RegisterPair;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Registers {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
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

    pub cycle_cooldown: u8,
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
            cycle_cooldown: 0,
        }
    }

    pub fn reset(&mut self) {
        // TODO: reset bus as well
        *self = CPU::new(self.should_trace_log);
    }

    fn decode(&mut self, opcode: u8) -> Instruction {
        match opcode {
            0x00 => instruction::control::nop,
            0o363 => instruction::control::di,
            0o373 => instruction::control::ei,
            0o166 | 0o20 => instruction::control::halt,

            0o30 => instruction::jump::jr_e8,
            0o40 | 0o50 | 0o60 | 0o70 => instruction::jump::jr_cc_e8,
            0o311 => instruction::jump::ret,
            0o331 => instruction::jump::reti,
            0o300 | 0o310 | 0o320 | 0o330 => instruction::jump::ret,
            0o351 => instruction::jump::jp_hl,
            0o303 => instruction::jump::jp_a16,
            0o302 | 0o312 | 0o322 | 0o332 => instruction::jump::jp_cc_a16,
            0o315 => instruction::jump::call,
            0o304 | 0o314 | 0o324 | 0o334 => instruction::jump::call_cc,
            0o307 | 0o317 | 0o327 | 0o337 | 0o347 | 0o357 | 0o367 | 0o377 => instruction::jump::rst,

            0o02 | 0o22 | 0o42 | 0o62 => instruction::load::ld_r16addr_a,
            0o12 | 0o32 | 0o52 | 0o72 => instruction::load::ld_a_r16addr,
            0o06 | 0o16 | 0o26 | 0o36 | 0o46 | 0o56 | 0o66 | 0o76 => instruction::load::ld_r8_n8,
            0o100..=0o165 | 0o167..=0o177 => instruction::load::ld_r8_r8,
            0o340 => instruction::load::ldh_a8addr_a,
            0o360 => instruction::load::ldh_a_a8addr,
            0o342 => instruction::load::ldh_caddr_a,
            0o352 => instruction::load::ld_a16addr_a,
            0o362 => instruction::load::ldh_a_caddr,
            0o372 => instruction::load::ld_a_a16addr,

            0o10 => instruction::load::ld_a16addr_sp,
            0o01 | 0o21 | 0o41 | 0o61 => instruction::load::ld_r16_n16,
            0o370 => instruction::load::ld_hl_sp_e8,
            0o371 => instruction::load::ld_sp_hl,
            0o301 | 0o321 | 0o341 | 0o361 => instruction::load::pop_r16,
            0o305 | 0o325 | 0o345 | 0o365 => instruction::load::push_r16,

            0o04 | 0o14 | 0o24 | 0o34 | 0o44 | 0o54 | 0o64 | 0o74 => instruction::logic::inc_r8,
            0o05 | 0o15 | 0o25 | 0o35 | 0o45 | 0o55 | 0o65 | 0o75 => instruction::logic::dec_r8,
            0o47 => instruction::logic::daa,
            0o57 => instruction::logic::cpl,
            0o67 => instruction::logic::scf,
            0o77 => instruction::logic::ccf,
            0o200..=0o207 => instruction::logic::add_a_r8,
            0o210..=0o217 => instruction::logic::adc_a_r8,
            0o220..=0o227 => instruction::logic::sub_a_r8,
            0o230..=0o237 => instruction::logic::sbc_a_r8,
            0o240..=0o247 => instruction::logic::and_a_r8,
            0o250..=0o257 => instruction::logic::xor_a_r8,
            0o260..=0o267 => instruction::logic::or_a_r8,
            0o270..=0o277 => instruction::logic::cp_a_r8,
            0o306 => instruction::logic::add_a_n8,
            0o316 => instruction::logic::adc_a_n8,
            0o326 => instruction::logic::sub_a_n8,
            0o336 => instruction::logic::sbc_a_n8,
            0o346 => instruction::logic::and_a_n8,
            0o356 => instruction::logic::xor_a_n8,
            0o366 => instruction::logic::or_a_n8,
            0o376 => instruction::logic::cp_a_n8,

            0o11 | 0o31 | 0o51 | 0o71 => instruction::logic::add_hl_r16,
            0o03 | 0o23 | 0o43 | 0o63  => instruction::logic::inc_r16,
            0o13 | 0o33 | 0o53 | 0o73 => instruction::logic::dec_r16,
            0o350 => instruction::logic::sp_e8,
            
            0o07 => instruction::bit::rlca,
            0o17 => instruction::bit::rrca,
            0o27 => instruction::bit::rla,
            0o37 => instruction::bit::rra,

            0o323 | 0o333 | 0o343 | 0o353 | 0o344 | 0o354 | 0o364 | 0o374 | 0o335 | 0o355 | 0o375 => instruction::empty,

            _ => {
                unimplemented!("Opcode {:02X} not implemented yet", opcode);
            }
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        if self.is_halting {
            return;
        }
        
        if self.cycle_cooldown > 0 {
            self.cycle_cooldown -= 1;
            return;
        }

        if self.ie_enable_delay {
            self.ie_enable_delay = false;
            bus.interrupts.ime = true;
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
        let (length, cycles) = instruction(self, bus);

        self.pc += length;
        self.cycle_cooldown = cycles;

        // if self.should_trace_log {
        //     if let Some(disasm) = disassemble(&*bus, self.pc) {
        //         println!(
        //             "{:04X}: {:<12} | A:{:02X} F:{}{}{}{} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X}",
        //             disasm.address,
        //             disasm.mnemonic,
        //             self.a,
        //             if self.flags.zero { "Z" } else { "-" },
        //             if self.flags.subtraction { "N" } else { "-" },
        //             if self.flags.half_carry { "H" } else { "-" },
        //             if self.flags.carry { "C" } else { "-" },
        //             self.b, self.c, self.d, self.e, self.h, self.l,
        //             self.sp,
        //         );
        //     } else {
        //         println!("{:04X}: <undisassembled>", self.pc);
        //     }
        // }
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

    pub fn get_registers(&self) -> Registers {
        Registers {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            h: self.h,
            l: self.l,
            flags: self.flags,
            sp: self.sp,
            pc: self.pc,
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
