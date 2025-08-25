use super::cpu::CPU;
use super::bus::Bus;

pub mod control;

pub mod inc;
pub mod dec;
pub mod ld;
pub mod ldh;
pub mod jump;
pub mod add;
pub mod adc;
pub mod sub;
pub mod sbc;
pub mod and;
pub mod xor;
pub mod or;
pub mod cp;
pub mod call;
pub mod rst;
pub mod ret;
pub mod push;
pub mod pop;
pub mod rotate;
pub mod carry;
pub mod accumulator;

pub type Instruction = fn(&mut CPU, &mut Bus) -> (u8, u8);