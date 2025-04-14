use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterPair {
    BC,
    DE,
    HL,
    SP,
}

impl fmt::Display for RegisterPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RegisterPair::BC => "BC",
            RegisterPair::DE => "DE",
            RegisterPair::HL => "HL",
            RegisterPair::SP => "SP",
        };
        write!(f, "{}", s)
    }
}

pub fn get_register_pair_by_code(code: u8) -> RegisterPair {
    match code & 0b11 {
        0 => RegisterPair::BC,
        1 => RegisterPair::DE,
        2 => RegisterPair::HL,
        3 => RegisterPair::SP,
        _ => unreachable!(),
    }
}


