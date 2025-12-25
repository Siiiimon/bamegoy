use crate::emulator::Emulator;
use crate::emulator::host::handle::Handle;
use std::{env, fs};

pub mod emulator;

// this struct should basically represent the ui

struct BamegoyApp {
    emulator_handle: Handle,
}

impl BamegoyApp {
    pub fn new(rom_filepath: Option<String>, should_trace_log: bool) -> Self {
        let cartridge_rom: Vec<u8> = match rom_filepath {
            Some(p) => match fs::read(&p) {
                Err(e) => {
                    eprintln!("failed to read {:?}: {}", p, e);
                    vec![0; 0x8000]
                }
                Ok(c) => c,
            },
            None => vec![0; 0x8000],
        };

        let handle = Emulator::init(cartridge_rom, should_trace_log);

        Self {
            emulator_handle: handle,
        }
    }
}

fn main() {
    env_logger::init();

    // handling cli args is neither the main's responsiblity, nor the emulators
    // so like any other user input, it should be handled by the ui
    let mut args: Vec<String> = env::args().collect();

    let rom_filepath = args.pop();

    let mut should_trace_log = false;
    for arg in args.into_iter().skip(1) {
        if arg == "--trace" {
            should_trace_log = true;
        }
    }

    BamegoyApp::new(rom_filepath, should_trace_log);
}
