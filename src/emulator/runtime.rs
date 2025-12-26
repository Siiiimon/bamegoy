pub mod bus;
pub mod cpu;
pub mod disassemble;
pub mod instruction;

use crate::emulator::host::{EmulatorMessage, policy::Policy};
use crate::emulator::runtime::{bus::Bus, cpu::CPU};

pub struct Runtime {
    state: State,
    policy: Option<Policy>,

    cpu: CPU,
    bus: Bus,
}

#[derive(PartialEq)]
pub enum State {
    Paused,
    Running,
}

impl Runtime {
    pub fn new(cpu: CPU, bus: Bus) -> Self {
        Self {
            state: State::Paused,
            policy: None,
            cpu,
            bus,
        }
    }

    pub fn transition_to(&mut self, new_state: State, new_policy: Option<Policy>) {
        self.state = new_state;
        self.policy = new_policy;
    }

    pub fn handle_current_state(&mut self) -> Option<EmulatorMessage> {
        match self.state {
            State::Paused => None,
            State::Running => {
                self.cpu.step(&mut self.bus);

                if let Some(p) = &mut self.policy {
                    if p(&self.cpu, &self.bus) {
                        self.policy = None;
                        self.state = State::Paused;
                        return Some(EmulatorMessage::Paused);
                    }
                }

                None
            }
        }
    }
}
