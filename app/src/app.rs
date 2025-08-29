use bamegoy::emulator::{Handle, EmulatorState, EmulatorMessage};

pub mod ui;

struct BamegoyApp {
    emulator_handle: Handle,
    app_state: AppState,
}

pub struct AppState {
    emulator_state: EmulatorState,
    last_pc: u16,
    bottom_panel_selected_tab: usize,
    views: Views,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            emulator_state: EmulatorState::Paused,
            last_pc: 0,
            bottom_panel_selected_tab: 0,
            settings_view: SettingsView::default(),
            disassembly_view: DisassemblyView::default(),
            breakpoint_view: BreakpointView::default(),
        }
    }
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
            ui_state: AppState::default(),
        }
    }

    fn handle_emulator_message(&mut self, msg: EmulatorMessage) {
        match msg {
            EmulatorMessage::Paused => {
                self.ui_state.emulator_state = EmulatorState::Paused;
            },
            EmulatorMessage::Running => {
                self.ui_state.emulator_state = EmulatorState::Running;
            },
            _ => panic!("uncovered message")
        }
    }
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
