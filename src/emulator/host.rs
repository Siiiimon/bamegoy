use std::sync::{mpsc::{Receiver, Sender, TryRecvError}, Arc, Mutex};

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
        let message = match self.receiver.try_recv() {
            Ok(m) => m,
            Err(err) => {
                if err == TryRecvError::Empty {
                    return;
                }
                panic!("{}", err)
            },
        };


        match message {
            // fixme: use proper module api instead of plainly mutating runtime state
            DriverMessage::Run(policy) => {
                self.runtime.state = State::Running;
                self.runtime.policy = policy;
                self.runtime.tx.send(EmulatorMessage::Running).unwrap();
            }
            DriverMessage::PauseRequest => {
                self.runtime.state = State::PauseRequested;
            }
        }
    }
}
