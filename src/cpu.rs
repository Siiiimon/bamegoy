use crate::bus;
use crate::instruction;
use crate::util::Register;
use crate::util::RegisterPair;
use crate::util::get_register_pair_by_code;

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

    pub bus: bus::SharedBus,
}

impl CPU {
    pub fn new(bus: bus::SharedBus) -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flags: Flags {
                zero: false,
                subtraction: false,
                half_carry: false,
                carry: false,
            },
            sp: 0,
            pc: 0x0100,
            bus,
        }
    }

    pub fn step(&mut self) {
        // fetch
        let opcode = match self.bus.borrow().rom_read_byte(self.pc) {
            Some(byte) => byte,
            None => {
                eprintln!("Tried to read invalid ROM address: {:04X}", self.pc);
                return;
            }
        };

        // decode
        match opcode {
            0x00 => {
                self.pc += 1;
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
                instruction::inc::r8(self, opcode);
            }
            0o05 | 0o15 | 0o25 | 0o35 | 0o45 | 0o55 | 0o65 | 0o75 => {
                instruction::dec::r8(self, opcode);
            }
            0o06 | 0o16 | 0o26 | 0o36 | 0o46 | 0o56 | 0o66 | 0o76 => {
                instruction::ld::r8_n8(self, opcode);
            }
            0o100..=0o175 | 0o167..=0o177 => {
                instruction::ld::r8_r8(self, opcode);
            }
            0o01 | 0o21 | 0o41 | 0o61 => {
                instruction::ld::r16_n16(self, opcode);
            }
            0o02 | 0o22 => {
                instruction::ld::addr_of_r16_a(self, opcode);
            }
            0o12 | 0o32 => {
                instruction::ld::a_addr_of_r16(self, opcode);
            }
            0o42 => instruction::ld::addr_of_hl_a(self, true),
            0o62 => instruction::ld::addr_of_hl_a(self, false),
            0o200..=0o207 => {
                instruction::add::r8(self, opcode);
            }
            0o11 | 0o31 | 0o51 | 0o71 => {
                instruction::add::r16(self, opcode);
            }
            0o210..=0o217 => {
                instruction::adc::r8(self, opcode);
            }
            0o220..=0o227 => {
                instruction::sub::r8(self, opcode);
            }
            0o230..=0o237 => {
                instruction::sbc::r8(self, opcode);
            }
            0o240..=0o247 => {
                instruction::and::r8(self, opcode);
            }
            0o250..=0o257 => {
                instruction::xor::r8(self, opcode);
            }
            0o260..=0o267 => {
                instruction::or::r8(self, opcode);
            }
            0o270..=0o277 => {
                instruction::cp::r8(self, opcode);
            }
            0o303 => {
                instruction::jp::a16(self);
            }
            _ => {
                unimplemented!("Opcode {:02X} not implemented yet", opcode);
            }
        }
    }

    pub fn get_register(&self, register: Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::HL => self
                .bus
                .borrow()
                .rom_read_byte(((self.h as u16) << 8) | (self.l as u16))
                .unwrap(),
        }
    }

    pub fn set_register(&mut self, register: Register, val: u8) {
        match register {
            Register::A => self.a = val,
            Register::B => self.b = val,
            Register::C => self.c = val,
            Register::D => self.d = val,
            Register::E => self.e = val,
            Register::H => self.h = val,
            Register::L => self.l = val,
            Register::HL => self
                .bus
                .borrow_mut()
                .rom_write_byte(((self.h as u16) << 8) | (self.l as u16), val)
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
}
