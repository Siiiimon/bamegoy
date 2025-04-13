use crate::bus;

pub struct Flags {
    zero: bool,
    subtraction: bool,
    half_carry: bool,
    carry: bool,
}

#[derive(Copy, Clone, Debug)]
enum Register {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

pub struct CPU {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    flags: Flags,

    sp: u16,
    pc: u16,

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
        self.pc += 1;

        // decode
        match opcode {
            0x00 => {
                println!("NOP!")
            }
            0o04 | 0o14 | 0o24 | 0o34 | 0o44 | 0o54 | 0o64 | 0o74 => {
                // INC register
                let register_code = (opcode >> 3) & 0b111;
                let register = CPU::get_register_by_code(register_code);
                self.inc_register(register);
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

    pub fn print_registers(&self) {
        println!("=== CPU Registers ===");
        println!(
            "A:  {:02X}    F: Z={} N={} H={} C={}",
            self.a, self.flags.zero as u8, self.flags.subtraction as u8, self.flags.half_carry as u8, self.flags.carry as u8
        );
        println!("B:  {:02X}    C: {:02X}", self.b, self.c);
        println!("D:  {:02X}    E: {:02X}", self.d, self.e);
        println!("H:  {:02X}    L: {:02X}", self.h, self.l);
        println!("SP: {:04X}    PC: {:04X}", self.sp, self.pc);
        println!("======================\n");
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

    fn get_register_by_code(code: u8) -> Register {
        match code {
            0 => Register::B,
            1 => Register::C,
            2 => Register::D,
            3 => Register::E,
            4 => Register::H,
            5 => Register::L,
            6 => Register::HL,
            7 => Register::A,
            _ => unreachable!(),
        }
    }
}
