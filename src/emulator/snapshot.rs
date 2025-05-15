use std::any::Any;
use crate::emulator::bus::error::BusError;
use crate::emulator::bus::{io, Bus, BusView};
use crate::emulator::bus::io::serial::Serial;
use crate::emulator::cpu::{CpuView, Flags, CPU};
use crate::emulator::util::{Register, RegisterPair};

const BUS_SNAPSHOT_RADIUS: u16 = 128;

#[derive(Clone)]
pub struct Snapshot {
    pub cpu: Box<dyn CpuView + Send>,
    pub bus: Box<dyn BusView + Send>,
}

impl Snapshot {
    pub fn from(cpu: &CPU, bus: &mut Bus) -> Self {
        Self {
            cpu: Box::new(CPUSnapshot {
                a: cpu.get_register(Register::A),
                b: cpu.get_register(Register::B),
                c: cpu.get_register(Register::C),
                d: cpu.get_register(Register::D),
                e: cpu.get_register(Register::E),
                h: cpu.get_register(Register::H),
                l: cpu.get_register(Register::L),
                flags: cpu.get_flags(),
                sp: cpu.get_register_pair(RegisterPair::SP),
                pc: cpu.get_pc(),
                is_halting: cpu.is_halting(),
            }),
            bus: Box::new(BusSnapshot {
                base: cpu.get_pc(),
                memory: {
                    let center = cpu.get_pc();
                    let start = center.saturating_sub(BUS_SNAPSHOT_RADIUS);
                    let end = center.saturating_add(BUS_SNAPSHOT_RADIUS);

                    let mut mem = Vec::new();
                    for addr in start..=end {
                        match bus.read_byte(addr) {
                            Ok(byte) => mem.push(byte),
                            Err(_) => mem.push(0xFF), // fallback if unmapped/bad read
                        }
                    }
                    mem.into_boxed_slice()
                },
                serial: bus.serial.clone(),
                interrupts: bus.interrupts.clone(),
            }),
        }
    }
}

#[derive(Clone)]
pub struct CPUSnapshot {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    flags: Flags,

    sp: u16,
    pc: u16,

    is_halting: bool,
}

impl CpuView for CPUSnapshot {
    fn as_any_mut(&mut self) -> &dyn Any {
        self
    }
    fn get_register(&self, register: Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
        }
    }

    fn set_register(&mut self, _bus: &mut Bus, _register: Register, _val: u8) {
        eprintln!("Tried to write to read-only cpu snapshot!");
    }

    fn get_register_pair(&self, pair: RegisterPair) -> u16 {
        match pair {
            RegisterPair::BC => ((self.b as u16) << 8) | (self.c as u16),
            RegisterPair::DE => ((self.d as u16) << 8) | (self.e as u16),
            RegisterPair::HL => ((self.h as u16) << 8) | (self.l as u16),
            RegisterPair::SP => self.sp,
        }
    }

    fn set_register_pair(&mut self, _pair: RegisterPair, _val: u16) {
        eprintln!("Tried to write to read-only cpu snapshot!");
    }

    fn get_flags(&self) -> Flags {
        self.flags.clone()
    }

    fn get_pc(&self) -> u16 {
        self.pc
    }

    fn is_halting(&self) -> bool {
        self.is_halting
    }
}

#[derive(Clone)]
pub struct BusSnapshot {
    base: u16,
    memory: Box<[u8]>,
    serial: Serial,
    interrupts: io::interrupts::Interrupts,
}

impl BusView for BusSnapshot {
    fn read_byte(&self, addr: u16) -> Result<u8, BusError> {
        let index = addr - self.base;
        if index as usize >= self.memory.len() { return Err(BusError::OutOfBounds(index)); }
        Ok(self.memory[index as usize])
    }

    fn write_byte(&mut self, _addr: u16, _content: u8) -> Result<(), BusError> {
        Err(BusError::WriteToSnapshot())
    }

    fn read_word(&self, addr: u16) -> Result<u16, BusError> {
        let lo = self.read_byte(addr)?;
        let hi = self.read_byte(addr + 1)?;

        Ok(((hi as u16) << 8) | lo as u16)
    }

    fn write_word(&mut self, _addr: u16, _content: u16) -> Result<(), BusError> {
        Err(BusError::WriteToSnapshot())
    }

    fn get_serial(&self) -> &Serial {
        &self.serial
    }
}