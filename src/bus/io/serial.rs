use crate::bus::error::BusError;

use super::interrupts::Interrupt;

#[derive(Default)]
pub struct Serial {
    pub content: u8,
    pub control: Control,
    pub outgoing: Vec<u8>,
}

impl Serial {
    pub fn read(&self, addr: u16) -> Result<u8, BusError> {
        if addr == 0xFF01 {
            return Ok(self.content);
        }
        Ok(self.control.to_byte())
    }

    pub fn write(&mut self, addr: u16, content: u8, interrupt: &mut Interrupt) -> Result<(), BusError> {
        if addr == 0xFF01 {
            self.content = content;
        }
        self.control.set(content);
        if self.control.enable {
            self.transfer(interrupt);
        }
        Ok(())
    }

    fn transfer(&mut self, interrupt: &mut Interrupt) {
        if !self.control.should_use_internal_clock {
            unimplemented!("use of external serial transfer has not been implemented yet");
        }
        self.outgoing.push(self.content);
        self.control.enable = false;
        interrupt.is_requested = true;
    }
}

#[derive(Default)]
pub struct Control {
    pub enable: bool,
    // is_high_speed: bool,
    pub should_use_internal_clock: bool, // clock_select
}


impl Control {
    fn to_byte(&self) -> u8 {
        let mut value = 0;
        if self.enable {
            value |= 0b1000_0000;
        }
        if self.should_use_internal_clock {
            value |= 1;
        }
        value
    }

    fn set(&mut self, content: u8) {
        self.enable = content >> 7 == 1;
        self.should_use_internal_clock = content & 1 == 1;
    }
}
