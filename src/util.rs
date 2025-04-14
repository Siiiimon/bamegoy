use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Register {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Register::A => "A",
            Register::B => "B",
            Register::C => "C",
            Register::D => "D",
            Register::E => "E",
            Register::H => "H",
            Register::L => "L",
            Register::HL => "(HL)",
        };
        write!(f, "{}", s)
    }
}

pub fn get_register_by_code(code: u8) -> Register {
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
