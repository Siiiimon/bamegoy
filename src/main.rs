use bamegoy::emulator::{Emulator, EmulatorMessage, Handle, EmulatorState};
use eframe::egui;
use std::sync::mpsc::TryRecvError;
use std::{env, fs};
use ui::{breakpoints::BreakpointView, disasm::DisassemblyView, settings::SettingsView};

mod app;

const COLORS: catppuccin::FlavorColors = catppuccin::PALETTE.macchiato.colors;

fn main() -> eframe::Result {
    env_logger::init();
    let mut args: Vec<String> = env::args().collect();

    let rom_filepath = args.pop();

    let mut should_trace_log = false;
    for arg in args.into_iter().skip(1) {
        if arg == "--trace" {
            should_trace_log = true;
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([680.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "bamegoy",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(BamegoyApp::new(rom_filepath, should_trace_log)))
        }),
    )
}