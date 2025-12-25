use crate::emulator::bus::Bus;
use crate::emulator::cpu::CPU;
use crate::emulator::host::{DriverMessage, EmulatorMessage, Handle, Host};
use crate::emulator::runtime::Runtime;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;

pub mod bus;
pub mod cpu;
pub mod disassemble;
pub mod instruction;
pub mod host;
pub mod runtime;
pub mod util;

pub struct Emulator {
    host: host::Host,
    runtime: runtime::Runtime,
    bus: bus::SharedBus,
    cpu: Arc<Mutex<CPU>>,
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
            runtime: Runtime::new(),

            bus: Arc::new(Mutex::new(bus)),
            cpu: Arc::new(Mutex::new(cpu)),
        }
    }



    fn live(&mut self) {
        // todo: when done writing a runtime and host api
        // self.runtime.tx.send(EmulatorMessage::Paused).unwrap();

        // loop {
        //     self.handle_driver_message();
        //
        //     self.handle_state();
        // }
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
