use super::error::BusError;

pub mod serial;
pub mod interrupts;

trait IOReadable {
    fn read(&self, addr: u16) -> Result<u8, BusError>;
}

trait IOWritable {
    fn write(&mut self, addr: u16, content: u8) -> Result<(), BusError>;
}

pub struct IORegisters {
    serial: serial::Serial,
    pub interrupts: interrupts::Interrupts,
}

impl IORegisters {
    pub fn new() -> Self {
        Self {
            serial: serial::Serial::default(),
            interrupts: interrupts::Interrupts::default()
        }
    }

    pub fn read(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0xFF01 | 0xFF02 => self.serial.read(addr),
            0xFF0F | 0xFFFF => self.interrupts.read(addr),
            _ => Err(BusError::Unimplemented(addr))
        }
    }

    pub fn write(&mut self, addr: u16, content: u8) -> Result<(), BusError> {
        match addr {
            0xFF01 | 0xFF02 => self.serial.write(addr, content),
            0xFF0F | 0xFFFF => self.interrupts.write(addr, content),
            _ => Err(BusError::Unimplemented(addr))
        }
    }
}
