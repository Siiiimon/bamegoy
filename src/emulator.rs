use crate::emulator::bus::Bus;
use crate::emulator::cpu::{Registers, CPU};
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use crate::emulator::policy::Policy;

pub mod bus;
pub mod cpu;
pub mod instruction;
pub mod disassemble;
pub mod util;
pub mod policy;

pub struct Emulator {
    runtime: Runtime,
    bus: Bus,
    cpu: CPU,
}

pub struct Runtime {
    state: EmulatorState,
    last_step_time: Instant,
    step_interval: Duration,
    should_exit: bool,

    tx: Sender<EmulatorMessage>,
    rx: Receiver<DriverMessage>,
    policy: Option<Policy>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EmulatorState {
    PauseRequested,
    Paused,
    Running,
    Dying,
}

pub struct Handle {
    pub thread: JoinHandle<()>,
    pub tx: Sender<DriverMessage>,
    pub rx: Receiver<EmulatorMessage>,
}

pub enum DriverMessage {
    Run(Option<Policy>),
    PauseRequest,
    Kill,

    GetRegisters,
}

#[derive(Debug, PartialEq)]
pub enum EmulatorMessage {
    Paused,
    Running,
    Registers(Registers),
}

impl Emulator {
     fn new(cartridge: Vec<u8>, should_trace: bool, tx: Sender<EmulatorMessage>, rx: Receiver<DriverMessage>) -> Self {
         let cpu = CPU::new(should_trace);
         let bus = Bus::from_cartridge_rom(cartridge).unwrap();

        Self {
            runtime: Runtime {
                state: EmulatorState::Paused,
                last_step_time: Instant::now(),
                step_interval: Duration::from_millis(100),
                should_exit: false,
                tx,
                rx,
                policy: None,
            },

            bus,
            cpu,
        }
    }

    fn handle_driver_message(&mut self) {
        let msg = self.runtime.rx.try_recv();

        if let Err(e) = msg {
            if e == TryRecvError::Empty {
                return;
            } else {
                panic!("{}", e);
            }
        }

        match msg.unwrap() {
            DriverMessage::Run(policy) => {
                self.runtime.state = EmulatorState::Running;
                self.runtime.policy = policy;
                self.runtime.tx.send(EmulatorMessage::Running).unwrap();
            },
            DriverMessage::PauseRequest => {
                self.runtime.state = EmulatorState::PauseRequested;
            },
            DriverMessage::Kill => {
                self.runtime.state = EmulatorState::Dying;
            },
            DriverMessage::GetRegisters => {
                let _ = self.runtime.tx.send(
                    EmulatorMessage::Registers(self.cpu.get_registers())
                );
            },
        }
    }

    fn handle_state(&mut self) {
        match self.runtime.state {
            EmulatorState::PauseRequested => {
                self.runtime.state = EmulatorState::Paused;
                self.runtime.tx.send(EmulatorMessage::Paused).unwrap();
            }
            EmulatorState::Paused => {}
            EmulatorState::Running => {
                self.cpu.step(&mut self.bus);

                if let Some(policy) = &mut self.runtime.policy {
                    if policy(&mut self.cpu, &mut self.bus) {
                        self.runtime.policy = None;
                        self.runtime.state = EmulatorState::PauseRequested;
                    }
                }
            },
            EmulatorState::Dying => {
                self.runtime.should_exit = true;
            }
        }
    }

    fn live(&mut self) {
        self.runtime.tx.send(EmulatorMessage::Paused).unwrap();

        while !self.runtime.should_exit {
            self.handle_driver_message();

            self.handle_state();            
        }
    }

    pub fn init(cartridge: Vec<u8>, should_trace: bool) -> Handle {
        let (driver_tx, driver_rx) = channel();
        let (emulator_tx, emulator_rx) = channel();

        let emulator = Self::new(cartridge, should_trace, emulator_tx, driver_rx);

        let thread = thread::spawn(move || {
            let mut emulator = emulator;
            emulator.live();
        });

        Handle {
            thread,
            tx: driver_tx,
            rx: emulator_rx,
        }
    }
}