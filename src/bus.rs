use std::{cell::RefCell, rc::Rc};

pub type SharedBus = Rc<RefCell<Bus>>;

pub struct Bus {
    pub rom: Vec<u8> // bank 0 and bank 1+ as one array
}

impl Bus {
    pub fn new() -> Self {
        Self { rom: vec![0; 0x8000] }
    }

    pub fn rom_read_byte(&self, addr: u16) -> Option<u8> {
        if addr as usize > self.rom.len() { return None; }
        Some(self.rom[addr as usize])
    }

    pub fn rom_write_byte(&mut self, addr: u16, byte: u8) -> Result<(), String> {
        if addr as usize > self.rom.len() { return Err(format!("Address {:04X} is outside of ROM bounds", addr)); }
        self.rom[addr as usize] = byte;
        Ok(())
    }
}
