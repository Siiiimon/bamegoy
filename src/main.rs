use bamegoy::emulator::{Emulator, EmulatorMessage, Handle, State};
use eframe::egui;
use std::sync::mpsc::TryRecvError;
use std::{env, fs};
use ui::{breakpoints::BreakpointView, disasm::DisassemblyView, settings::SettingsView};

mod ui;

const COLORS: catppuccin::FlavorColors = catppuccin::PALETTE.macchiato.colors;

pub struct UiState {
    emulator_state: State,
    last_pc: u16,
    bottom_panel_selected_tab: usize,
    settings_view: SettingsView,
    disassembly_view: DisassemblyView,
    breakpoint_view: BreakpointView,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            emulator_state: State::Paused,
            last_pc: 0,
            bottom_panel_selected_tab: 0,
            settings_view: SettingsView::default(),
            disassembly_view: DisassemblyView::default(),
            breakpoint_view: BreakpointView::default(),
        }
    }
}

struct BamegoyApp {
    emulator_handle: Handle,
    ui_state: UiState,
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
            ui_state: UiState::default(),
        }
    }

    fn handle_emulator_message(&mut self, msg: EmulatorMessage) {
        match msg {
            EmulatorMessage::Paused => {
                self.ui_state.emulator_state = State::Paused;
            },
            EmulatorMessage::Running => {
                self.ui_state.emulator_state = State::Running;
            },
            _ => panic!("uncovered message")
        }
    }
}

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

impl eframe::App for BamegoyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.emulator_handle.rx.try_recv() {
            Ok(msg) => self.handle_emulator_message(msg),
            Err(e) => {
                if e != TryRecvError::Empty {
                    ui::draw_error(ctx, "Channel Disconnected".to_string());
                    return;
                }
            },
        }

        // if self.ui_state.emulator_state == State::Running {
        //     ui::draw_running(ctx, &mut self.emulator_handle);
        // } else {
        //     match (self.emulator_handle.cpu.try_lock(), self.emulator_handle.bus.try_lock()) {
        //         (Ok(mut cpu), Ok(mut bus)) => {
        //             ui::draw(ctx, &mut self.ui_state, self.emulator_handle.tx.clone(), &mut *bus, &mut *cpu);
        //         }
        //         (Err(TryLockError::WouldBlock), _) => ui::draw_error(ctx, "acquiring cpu handle...".to_string()),
        //         (_, Err(TryLockError::WouldBlock)) => ui::draw_error(ctx, "acquiring bus handle...".to_string()),
        //         (Err(TryLockError::Poisoned(_)), _) |
        //         (_, Err(TryLockError::Poisoned(_))) => {
        //             ui::draw_error(ctx, "CPU or Bus lock poisoned!".to_string());
        //         }
        //     }
        // }
    }
}
