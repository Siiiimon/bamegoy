use std::{cell::RefCell, rc::Rc};
use error::BusError;

mod error;
pub mod io;

pub type SharedBus = Rc<RefCell<Bus>>;

pub struct Bus {
    pub rom: Box<[u8]>,
    vram: Box<[u8]>,
    ram: Box<[u8]>,
    oam: Box<[u8]>,
    pub io: io::IORegisters,
    high_ram: Box<[u8]>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            rom: vec![0; 0x8000].into_boxed_slice(),
            vram: vec![0; 0x2000].into_boxed_slice(),
            ram: vec![0; 0x4000].into_boxed_slice(),
            oam: vec![0; 0xA0].into_boxed_slice(),
            io: io::IORegisters::new(),
            high_ram: vec![0; 127].into_boxed_slice(),
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0x0..0x8000 => {
                Self::mem_read(&self.rom, addr)
            }
            0x8000..0xA000 => {
                Self::mem_read(&self.vram, addr - 0x8000)
            }
            0xA000..0xE000 => {
                Self::mem_read(&self.ram, addr - 0xA000)
            }
            0xFE00..0xFEA0 => {
                Self::mem_read(&self.oam, addr - 0xFE00)
            }
            0xFF00..0xFF80 => {
                self.io.read(addr)
            }
            0xFF80..=0xFFFE => {
                Self::mem_read(&self.high_ram, addr - 0xFF80)
            }
            _ => Err(BusError::OutOfBounds(addr))
        }
    }

    pub fn write_byte(&mut self, addr: u16, content: u8) -> Result<(), BusError> {
        match addr {
            0x0..0x8000 => {
                Self::mem_write(&mut self.rom, addr, content)
            }
            0x8000..0xA000 => {
                Self::mem_write(&mut self.vram, addr - 0x8000, content)
            }
            0xA000..0xE000 => {
                Self::mem_write(&mut self.ram, addr - 0xA000, content)
            }
            0xFE00..0xFEA0 => {
                Self::mem_write(&mut self.oam, addr - 0xFE00, content)
            }
            0xFF00..0xFF80 => {
                self.io.write(addr - 0xFF00, content)
            }
            0xFF80..0xFFFD => {
                Self::mem_write(&mut self.high_ram, addr - 0xFF80, content)
            }
            _ => Err(BusError::OutOfBounds(addr))
        }
    }

    pub fn read_word(&self, addr: u16) -> Result<u16, BusError> {
        let lo = self.read_byte(addr)?;
        let hi = self.read_byte(addr + 1)?;

        Ok(((hi as u16) << 8) | lo as u16)
    }

    pub fn write_word(&mut self, addr: u16, content: u16) -> Result<(), BusError> {
        let hi = (content >> 8) as u8;
        let lo = content as u8;

        self.write_byte(addr, hi)?;
        self.write_byte(addr + 1, lo)?;

        Ok(())
    }

    fn mem_read(mem: &[u8], addr: u16) -> Result<u8, BusError> {
        if addr as usize >= mem.len() { return Err(BusError::OutOfBounds(addr)); }
        Ok(mem[addr as usize])
    }

    fn mem_write(mem: &mut [u8], addr: u16, content: u8) -> Result<(), BusError> {
        if addr as usize >= mem.len() { return Err(BusError::OutOfBounds(addr)); }
        mem[addr as usize] = content;
        Ok(())
    }

    pub fn push_word(&mut self, sp: &mut u16, content: u16) -> Result<(), BusError> {
        *sp -= 2;

        if (*sp as usize) + 1 >= self.rom.len() { return Err(BusError::OutOfBounds(*sp)) }
        self.write_word(*sp, content)

    }

    pub fn pop_word(&mut self, sp: &mut u16) -> Result<u16, BusError> {
        if (*sp as usize) + 1 >= self.rom.len() { return Err(BusError::OutOfBounds(*sp)) }

        let content = self.read_word(*sp)?;

        *sp += 2;

        Ok(content)
    }

    pub fn from_cartridge_rom(&mut self, cart: Vec<u8>) -> Result<(), String> {
        if cart.len() > self.rom.len() { return Err(format!("Cartridge rom too big!")); }
        self.rom[..cart.len()].copy_from_slice(&cart);
        Ok(())
    }
}
