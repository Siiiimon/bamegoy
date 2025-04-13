use std::{cell::RefCell, rc::Rc};
use crate::bus::Bus;
use crate::bus::SharedBus;

pub mod cpu;
pub mod bus;


fn main() {
    let bus: SharedBus = Rc::new(RefCell::new(Bus::new()));

    let cpu = cpu::CPU::new(bus);

    cpu.step();
}
