use crate::bus;
use crate::instruction;
use crate::util::Register;

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
}
