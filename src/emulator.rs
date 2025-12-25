use crate::emulator::bus::Bus;
use crate::emulator::cpu::CPU;
use crate::emulator::host::handle::Handle;
use crate::emulator::host::{DriverMessage, EmulatorMessage, Host};
use crate::emulator::runtime::{Runtime, State};
use std::{
    sync::mpsc::{Receiver, Sender, channel},
    thread,
};

pub mod bus;
pub mod cpu;
pub mod disassemble;
pub mod host;
pub mod instruction;
pub mod runtime;
pub mod util;

pub struct Emulator {
    host: host::Host,
    runtime: runtime::Runtime,
}

impl Emulator {
    // if you expected `new` to be `pub`, see the public `init`
    // method for a comment explaining why it isn't
    fn new(
        cartridge: Vec<u8>,
        should_trace: bool,
        sender: Sender<EmulatorMessage>,
        receiver: Receiver<DriverMessage>,
    ) -> Self {
        let cpu = CPU::new(should_trace);
        let bus = Bus::from_cartridge_rom(cartridge).unwrap();

        Self {
            host: Host::new(sender, receiver),
            runtime: Runtime::new(cpu, bus),
        }
    }

    fn live(&mut self) {
        self.host.emit_message(EmulatorMessage::Paused);
        self.runtime.transition_to(State::Paused, None);

        loop {
            self.host.handle_driver_message(&mut self.runtime);

            if let Some(message) = self.runtime.handle_current_state() {
                self.host.emit_message(message);
            }
        }
    }

    // usually, structs are instantiated with a public facing `new` or `default` method, which also
    // deal with any initialisation work. because the core is running in a separate thread from the
    // host system, the emulator has to be instantiated with references to sender / receive, core systems,
    // etc. which have to be created beforehand.
    // there is also the fact that we don't hand out the whole emulator instance (which would be the expected
    // `Self` in a `new` method), but only a `Handle`.
    // that's why we chose to highlight this quirk by having a pub `init` method instead.
    pub fn init(cartridge: Vec<u8>, should_trace: bool) -> Handle {
        let (driver_tx, driver_rx) = channel();
        let (emulator_tx, emulator_rx) = channel();

        let emulator = Self::new(cartridge, should_trace, emulator_tx, driver_rx);
        // fixme: instead of cloning a mutable arc to the frontend, we should have the host module
        // deal with an abstraction to these submodules
        // let cpu_arc = emulator.cpu.clone();
        // let bus_arc = emulator.bus.clone();

        thread::spawn(move || {
            let mut emulator = emulator;
            emulator.live();
        });

        Handle {
            tx: driver_tx,
            rx: emulator_rx,
        }
    }
}
