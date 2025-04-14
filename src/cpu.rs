use crate::bus;
use crate::util;
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

    bus: bus::SharedBus,
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
            pc: 0,
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
            0x00 => {}
            0o04 | 0o14 | 0o24 | 0o34 | 0o44 | 0o54 | 0o64 | 0o74 => {
                // INC register
                let register_code = (opcode >> 3) & 0b111;
                let register = util::get_register_by_code(register_code);
                self.inc_register(register);
                self.pc += 1;
            }
            0o05 | 0o15 | 0o25 | 0o35 | 0o45 | 0o55 | 0o65 | 0o75 => {
                // DEC register
                let register_code = (opcode >> 3) & 0b111;
                let register = util::get_register_by_code(register_code);
                self.dec_register(register);
                self.pc += 1;
            }
            0o06 | 0o16 | 0o26 | 0o36 | 0o46 | 0o56 | 0o66 | 0o76 => {
                // LD register
                let register_code = (opcode >> 3) & 0b111;
                let register = util::get_register_by_code(register_code);
                self.pc += 1;
                let value = match self.bus.borrow().rom_read_byte(self.pc) {
                    Some(byte) => byte,
                    None => {
                        eprintln!("Tried to read invalid ROM address: {:04X}", self.pc);
                        return;
                    }
                };
                self.set_register(register, value);
                self.pc += 1;
            }
            _ => {
                unimplemented!("Opcode {:02X} not implemented yet", opcode);
            }
        }
    }

    fn get_register(&self, register: Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::HL => unimplemented!("tried to get value of register HL"),
        }
    }

    fn set_register(&mut self, register: Register, val: u8) {
        match register {
            Register::A => self.a = val,
            Register::B => self.b = val,
            Register::C => self.c = val,
            Register::D => self.d = val,
            Register::E => self.e = val,
            Register::H => self.h = val,
            Register::L => self.l = val,
            Register::HL => unimplemented!("tried to set value for register HL"),
        }
    }

    fn inc_register(&mut self, register: Register) {
        let current = self.get_register(register);
        let new = current.wrapping_add(1);

        self.flags.zero = new == 0;
        self.flags.subtraction = false;
        self.flags.half_carry = (current & 0x0F) + 1 > 0x0F;
        self.flags.carry = current == u8::MAX;

        self.set_register(register, new);
    }

    fn dec_register(&mut self, register: Register) {
        let current = self.get_register(register);
        let new = current.wrapping_sub(1);

        self.flags.zero = new == 0;
        self.flags.subtraction = true;
        self.flags.half_carry = (current & 0x0F) == 0;
        self.flags.carry = current == u8::MAX;

        self.set_register(register, new);
    }
}
