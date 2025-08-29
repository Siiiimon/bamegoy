use crate::cpu::Registers;

#[derive(Debug, PartialEq)]
pub enum Event {
    Paused,
    Running,
    Registers(Registers),
}