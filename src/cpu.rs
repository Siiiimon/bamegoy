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
}

impl CPU {
    pub fn new() -> Self {
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
        }
    }
}
