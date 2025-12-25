use std::sync::{mpsc::{Receiver, Sender}, Arc, Mutex};

use crate::emulator::{bus::Bus, cpu::CPU};

pub mod policy;

pub enum DriverMessage {
    Run(Option<policy::Policy>),
    PauseRequest,
}

#[derive(Debug, PartialEq)]
pub enum EmulatorMessage {
    Paused,
    Running,
}

pub struct Handle {
    pub tx: Sender<DriverMessage>,
    pub rx: Receiver<EmulatorMessage>,
    pub cpu: Arc<Mutex<CPU>>,
    pub bus: Arc<Mutex<Bus>>,
}

pub struct Host {
    sender: Sender<EmulatorMessage>,
    receiver: Receiver<DriverMessage>,
    policy: Option<policy::Policy>,
}

impl Host {
    pub fn new(sender: Sender<EmulatorMessage>, receiver: Receiver<DriverMessage>) -> Self {
        Self {
            sender,
            receiver,
            policy: None,
        }
    }

    fn handle_driver_message(&mut self) {
        let message = self.receiver.try_recv();

        // fixme: make matcharm simpler, by panicing on err, returning on empty
        // and only deal with happy path in match arm without `Ok(...)` wrapping it's distracting

        match message {
            // fixme: use proper module api instead of plainly mutating runtime state
            Ok(DriverMessage::Run(policy)) => {
                self.runtime.state = State::Running;
                self.runtime.policy = policy;
                self.runtime.tx.send(EmulatorMessage::Running).unwrap();
            }
            Ok(DriverMessage::PauseRequest) => {
                self.runtime.state = State::PauseRequested;
            }
            Err(TryRecvError::Empty) => {}
            Err(e) => panic!("{}", e),
        }
    }
}
