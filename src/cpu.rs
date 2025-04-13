use crate::bus;

pub struct Flags {
    zero: bool,
    subtraction: bool,
    half_carry: bool,
    carry: bool,
}

pub struct CPU {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: Flags,

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
            f: Flags {
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

    pub fn step(mut self) {
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
            _ => {
                unimplemented!("Opcode {:02X} not implemented yet", opcode);
            }
        }
    }
}
