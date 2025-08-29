use crate::emulator::bus::error::BusError;

pub struct Interrupts {
    pub ime: bool,
    pub registers: [Interrupt; 5],
}

pub enum InterruptKind {
    VBlank = 0,
    LCDStat = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}

impl InterruptKind {
    pub fn from_bit_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(InterruptKind::VBlank),
            1 => Some(InterruptKind::LCDStat),
            2 => Some(InterruptKind::Timer),
            3 => Some(InterruptKind::Serial),
            4 => Some(InterruptKind::Joypad),
            _ => None,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Interrupt {
    pub is_enabled: bool,
    pub is_requested: bool,
}

impl Default for Interrupts {
    fn default() -> Self {
        Self {
            ime: false,
            registers: [Interrupt::default(); 5],
        }
    }
}

impl Interrupts {
    pub fn read(&self, addr: u16) -> Result<u8, BusError> {
        match addr {
            0xFF0F => {
                // IF
                let mut result = 0;
                for (i, int) in self.registers.iter().enumerate() {
                    if int.is_requested {
                        result |= 1 << i;
                    }
                }
                Ok(result)
            }
            0xFFFF => {
                // IE
                let mut result = 0;
                for (i, int) in self.registers.iter().enumerate() {
                    if int.is_enabled {
                        result |= 1 << i;
                    }
                }
                Ok(result)
            }
            _ => Err(BusError::Unimplemented(addr)),
        }
    }

    pub fn write(&mut self, addr: u16, content: u8) -> Result<(), BusError> {
        match addr {
            0xFF0F => {
                for i in 0..5 {
                    self.registers[i].is_requested = (content & (1 << i)) != 0;
                }
                Ok(())
            }
            0xFFFF => {
                for i in 0..5 {
                    self.registers[i].is_enabled = (content & (1 << i)) != 0;
                }
                Ok(())
            }
            _ => Err(BusError::Unimplemented(addr)),
        }
    }

    pub fn get(&self, kind: InterruptKind) -> &Interrupt {
        &self.registers[kind as usize]
    }

    pub fn get_mut(&mut self, kind: InterruptKind) -> &mut Interrupt {
        &mut self.registers[kind as usize]
    }
}
