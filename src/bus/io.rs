use super::error::BusError;

trait IOReadable {
    fn read(&self);
}

trait IOWritable {
    fn write(&mut self, content: u8);
}

pub struct IORegisters {
}

impl IORegisters {
    pub fn read(&self, addr: u16) -> Result<u8, BusError> {
        unimplemented!("reading from io address {} is not supported yet", addr);
    }

    pub fn write(&self, addr: u16, _content: u8) -> Result<(), BusError> {
        unimplemented!("writing to io address {} is not supported yet", addr);
    }
}
