use crate::emulator::bus::Bus;
use crate::emulator::cpu::CPU;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, TryLockError};
use std::thread;
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
    bus: bus::SharedBus,
    cpu: Arc<Mutex<CPU>>,
}

pub struct Runtime {
    state: State,
    last_step_time: Instant,
    step_interval: Duration,

    tx: Sender<EmulatorMessage>,
    rx: Receiver<DriverMessage>,
    policy: Option<Policy>,
}

#[derive(PartialEq)]
pub enum State {
    PauseRequested,
    Paused,
    Running,
}

pub struct Handle {
    pub tx: Sender<DriverMessage>,
    pub rx: Receiver<EmulatorMessage>,
    pub cpu: Arc<Mutex<CPU>>,
    pub bus: Arc<Mutex<Bus>>,
}

pub enum DriverMessage {
    Run(Option<Policy>),
    PauseRequest,
}

#[derive(Debug, PartialEq)]
pub enum EmulatorMessage {
    Paused,
    Running,
}

impl Emulator {
     fn new(cartridge: Vec<u8>, should_trace: bool, tx: Sender<EmulatorMessage>, rx: Receiver<DriverMessage>) -> Self {
         let cpu = CPU::new(should_trace);
         let bus = Bus::from_cartridge_rom(cartridge).unwrap();

        Self {
            runtime: Runtime {
                state: State::Paused,
                last_step_time: Instant::now(),
                step_interval: Duration::from_millis(100),
                tx,
                rx,
                policy: None,
            },

            bus: Arc::new(Mutex::new(bus)),
            cpu: Arc::new(Mutex::new(cpu)),
        }
    }

    fn live(&mut self) {
        self.runtime.tx.send(EmulatorMessage::Paused).unwrap();

        loop {
            let msg = self.runtime.rx.try_recv();
            match msg {
                Ok(DriverMessage::Run(policy)) => {
                    self.runtime.state = State::Running;
                    self.runtime.policy = policy;
                    self.runtime.tx.send(EmulatorMessage::Running).unwrap();
                }
                Ok(DriverMessage::PauseRequest) => {
                    self.runtime.state = State::PauseRequested;
                }
                Err(TryRecvError::Empty) => {},
                Err(e) => {
                    panic!("{}", e);
                }
            }

            match self.runtime.state {
                State::PauseRequested => {
                    self.runtime.state = State::Paused;
                    self.runtime.tx.send(EmulatorMessage::Paused).unwrap();
                }
                State::Paused => {}
                State::Running => {
                    match (self.cpu.try_lock(), self.bus.try_lock()) {
                        (Ok(mut cpu), Ok(mut bus)) => {
                            cpu.step(&mut *bus);
                            if let Some(p) = &mut self.runtime.policy {
                                if p(&*cpu, &*bus) {
                                    self.runtime.policy = None;
                                    self.runtime.state = State::PauseRequested;
                                }
                            }
                        }
                        (Err(TryLockError::WouldBlock), _) |
                        (_, Err(TryLockError::WouldBlock)) => {},
                        (Err(TryLockError::Poisoned(_)), _) |
                        (_, Err(TryLockError::Poisoned(_))) => {
                            panic!("CPU or Bus lock poisoned!");
                        }
                    }

                }
            }
        }
    }

    pub fn init(cartridge: Vec<u8>, should_trace: bool) -> Handle {
        let (driver_tx, driver_rx) = channel();
        let (emulator_tx, emulator_rx) = channel();

        let emulator = Self::new(cartridge, should_trace, emulator_tx, driver_rx);
        let cpu_arc = emulator.cpu.clone();
        let bus_arc = emulator.bus.clone();

        thread::spawn(move || {
            let mut emulator = emulator;
            emulator.live();
        });

        Handle {
            tx: driver_tx,
            rx: emulator_rx,
            cpu: cpu_arc,
            bus: bus_arc,
        }
    }
}