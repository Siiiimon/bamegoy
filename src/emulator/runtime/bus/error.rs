#[derive(Debug)]
pub enum BusError {
    OutOfBounds(u16),
    Unimplemented(u16),
}

impl std::fmt::Display for BusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BusError::OutOfBounds(addr) => {
                write!(f, "Address {:04X} is outside of ROM bounds", addr)
            }
            BusError::Unimplemented(addr) => {
                write!(f, "reading from io address {} is not supported yet", addr)
            }
        }
    }
}

impl std::error::Error for BusError {}
