use std::{cell::RefCell, rc::Rc};
use crate::bus::Bus;
use crate::bus::SharedBus;

pub mod cpu;
pub mod bus;


fn main() {
    let bus: SharedBus = Rc::new(RefCell::new(Bus::new()));

    let _ = bus.borrow_mut().rom_write_byte(0, 0o04);
    let _ = bus.borrow_mut().rom_write_byte(1, 0o14);
    let _ = bus.borrow_mut().rom_write_byte(2, 0o24);
    let _ = bus.borrow_mut().rom_write_byte(3, 0o34);
    let _ = bus.borrow_mut().rom_write_byte(4, 0o44);
    let _ = bus.borrow_mut().rom_write_byte(5, 0o54);
    let _ = bus.borrow_mut().rom_write_byte(6, 0o74);

    let mut cpu = cpu::CPU::new(bus.clone());

    for _ in 0..7 {
        cpu.step();
    }

    cpu.print_registers();
}
