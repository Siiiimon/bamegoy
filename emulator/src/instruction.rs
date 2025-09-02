use super::cpu::CPU;
use super::bus::Bus;

pub mod control;
pub mod jump;
pub mod load;
pub mod logic;
pub mod bit;

pub type Instruction = fn(&mut CPU, &mut Bus) -> (u16, u8);

pub fn empty(_cpu: &mut CPU, _bus: &mut Bus) -> (u16, u8) {
    (0, 0)
}

pub fn get_opcode(cpu: &mut CPU, bus: &mut Bus) -> u8 {
    match bus.read_byte(cpu.pc) {
        Ok(op) => op,
        Err(e) => {
            panic!("{}", e);
        }
    }
}