use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use crate::emulator::runtime::{Runtime, State};

pub mod handle;
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

pub struct Host {
    sender: Sender<EmulatorMessage>,
    receiver: Receiver<DriverMessage>,
}

impl Host {
    pub fn new(sender: Sender<EmulatorMessage>, receiver: Receiver<DriverMessage>) -> Self {
        Self { sender, receiver }
    }

    pub fn emit_message(&mut self, message: EmulatorMessage) {
        self.sender.send(message).unwrap();
    }

    pub fn handle_driver_message(&mut self, runtime: &mut Runtime) {
        let message = match self.receiver.try_recv() {
            Ok(m) => m,
            Err(err) => {
                if err == TryRecvError::Empty {
                    return;
                }
                panic!("{}", err)
            }
        };

        match message {
            DriverMessage::Run(policy) => {
                runtime.transition_to(State::Running, policy);
                self.emit_message(EmulatorMessage::Running);
            }
            DriverMessage::PauseRequest => {
                runtime.transition_to(State::Paused, None);
                self.emit_message(EmulatorMessage::Paused);
            }
        }
    }
}
