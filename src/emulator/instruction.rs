use super::cpu::CPU;
use super::bus::Bus;

pub mod control;
pub mod jump;
pub mod load;
pub mod logic;

pub mod inc;
pub mod dec;
pub mod ld;
pub mod ldh;
pub mod add;
pub mod adc;
pub mod sub;
pub mod sbc;
pub mod and;
pub mod xor;
pub mod or;
pub mod cp;
pub mod push;
pub mod pop;
pub mod rotate;
pub mod carry;
pub mod accumulator;

pub type Instruction = fn(&mut CPU, &mut Bus) -> (u16, u8);

pub fn get_opcode(cpu: &mut CPU, bus: &mut Bus) -> u8 {
    match bus.read_byte(cpu.pc) {
        Ok(op) => op,
        Err(e) => {
            panic!("{}", e);
        }
    }
}