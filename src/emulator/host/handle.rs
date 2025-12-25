use std::sync::mpsc::{Receiver, Sender};

use crate::emulator::host::{DriverMessage, EmulatorMessage};

pub struct Handle {
    pub tx: Sender<DriverMessage>,
    pub rx: Receiver<EmulatorMessage>,
}
