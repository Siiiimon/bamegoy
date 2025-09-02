use std::sync::Arc;
use crossbeam::atomic::AtomicCell;
use crate::bus::Bus;
use crate::cpu::{Registers, CPU};
use crate::protocol::command::Command;
use crate::protocol::event::Event;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Instant;
use crate::protocol::policy::Policy;

pub mod bus;
pub mod cpu;
pub mod instruction;
pub mod util;
pub mod protocol;
mod timing;

pub struct Emulator {
    runtime: Runtime,
    bus: Bus,
    cpu: CPU,
}

pub struct Runtime {
    state: EmulatorState,
    drift: f64,
    should_exit: bool,

    tx: Sender<Event>,
    rx: Receiver<Command>,
    policy: Option<Policy>,

    register_snapshot: Arc<AtomicCell<Registers>>
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
    pub tx: Sender<Command>,
    pub rx: Receiver<Event>,
    pub register_snapshot: Arc<AtomicCell<Registers>>
}

impl Emulator {
     fn new(cartridge: Vec<u8>, should_trace: bool, tx: Sender<Event>, rx: Receiver<Command>) -> Self {
         let cpu = CPU::new(should_trace);
         let bus = Bus::from_cartridge_rom(cartridge).unwrap();

        Self {
            runtime: Runtime {
                state: EmulatorState::Paused,
                drift: 0.0,
                should_exit: false,
                tx,
                rx,
                policy: None,
                register_snapshot: Arc::new(AtomicCell::new(Registers::default())),
            },

            bus,
            cpu,
        }
    }

    fn handle_command(&mut self) {
        let msg = self.runtime.rx.try_recv();

        if let Err(e) = msg {
            if e == TryRecvError::Empty {
                return;
            } else {
                panic!("{}", e);
            }
        }

        match msg.unwrap() {
            Command::Run(policy) => {
                self.runtime.state = EmulatorState::Running;
                self.runtime.policy = policy;
                self.runtime.tx.send(Event::Running).unwrap();
            },
            Command::PauseRequest => {
                self.runtime.state = EmulatorState::PauseRequested;
            },
            Command::Kill => {
                self.runtime.state = EmulatorState::Dying;
            },
            Command::GetRegisters => {
                let _ = self.runtime.tx.send(
                    Event::Registers(self.cpu.get_registers())
                );
            },
        }
    }

    fn handle_state(&mut self) {
        match self.runtime.state {
            EmulatorState::PauseRequested => {
                self.runtime.state = EmulatorState::Paused;
                self.runtime.tx.send(Event::Paused).unwrap();
            }
            EmulatorState::Paused => {}
            EmulatorState::Running => {
                self.advance();
            },
            EmulatorState::Dying => {
                self.runtime.should_exit = true;
            }
        }
    }

    fn advance(&mut self) {
        let start = Instant::now();

        for _ in 0..timing::CYCLES_PER_SLICE-1 {
            self.cpu.step(&mut self.bus);

            if let Some(policy) = &mut self.runtime.policy {
                if policy(&mut self.cpu, &mut self.bus) {
                    self.runtime.policy = None;
                    self.runtime.state = EmulatorState::PauseRequested;
                    break;
                }
            }
        }

        self.runtime.register_snapshot.store(self.cpu.get_registers());

        self.runtime.drift += timing::calculate_drift(start);
        timing::nap(&mut self.runtime.drift);
    }

    fn live(&mut self) {
        self.runtime.tx.send(Event::Paused).unwrap();

        while !self.runtime.should_exit {
            self.handle_command();

            self.handle_state();
        }
    }

    pub fn init(cartridge: Vec<u8>, should_trace: bool) -> Handle {
        let (driver_tx, driver_rx) = channel();
        let (emulator_tx, emulator_rx) = channel();

        let emulator = Self::new(cartridge, should_trace, emulator_tx, driver_rx);
        let register_snapshot = emulator.runtime.register_snapshot.clone();

        let thread = thread::spawn(move || {
            let mut emulator = emulator;
            emulator.live();
        });

        Handle {
            thread,
            tx: driver_tx,
            rx: emulator_rx,
            register_snapshot,
        }
    }
}