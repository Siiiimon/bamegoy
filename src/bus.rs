use std::{cell::RefCell, rc::Rc};
use error::BusError;

mod error;

pub type SharedBus = Rc<RefCell<Bus>>;

impl std::error::Error for BusError {}

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

    pub fn rom_read_word(&self, addr: u16) -> Option<u16> {
        if (addr as usize) + 1 >= self.rom.len() { return None; }

        let hi = self.rom[(addr + 1) as usize] as u16;
        let lo = self.rom[addr as usize] as u16;

        Some((hi << 8) | lo)
    }

    pub fn push_word(&mut self, sp: &mut u16, content: u16) -> Result<(), BusError> {
        *sp -= 2;

        if (*sp as usize) + 1 >= self.rom.len() { return Err(BusError::OutOfBounds(*sp)) }
        self.rom_write_word(*sp, content)?;

        Ok(())
    }

    pub fn pop_word(&mut self, sp: &mut u16) -> Result<u16, BusError> {
        if (*sp as usize) + 1 >= self.rom.len() { return Err(BusError::OutOfBounds(*sp)) }

        let content = self.rom_read_word(*sp).ok_or(BusError::OutOfBounds(*sp))?;

        *sp += 2;

        Ok(content)
    }

    pub fn rom_write_byte(&mut self, addr: u16, byte: u8) -> Result<(), BusError> {
        if addr as usize >= self.rom.len() { return Err(BusError::OutOfBounds(addr)); }
        self.rom[addr as usize] = byte;
        Ok(())
    }

    pub fn rom_write_word(&mut self, addr: u16, word: u16) -> Result<(), BusError> {
        if (addr as usize) + 1 >= self.rom.len() { return Err(BusError::OutOfBounds(addr)); }

        let hi = (word >> 8) as u8;
        let lo = word as u8;

        self.rom[addr as usize] = lo;
        self.rom[(addr as usize) + 1] = hi;

        Ok(())
    }

    pub fn from_cartridge_rom(&mut self, cart: Vec<u8>) -> Result<(), String> {
        if cart.len() > self.rom.len() { return Err(format!("Cartridge rom too big!")); }
        self.rom[..cart.len()].copy_from_slice(&cart);
        Ok(())
    }
}
