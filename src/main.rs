use emulator::bus::Bus;
use emulator::disassemble::disassemble;
use eframe::egui;
use std::{cell::RefCell, env, fs, rc::Rc, time::{Duration, Instant}};
use ui::{breakpoints::BreakpointView, disasm::DisassemblyView, draw_memory_panel, draw_serial_panel, settings::SettingsView};

pub mod emulator;

mod ui;

const COLORS: catppuccin::FlavorColors = catppuccin::PALETTE.macchiato.colors;

pub struct UiState {
    last_pc: u16,
    bottom_panel_selected_tab: usize,
    settings_view: SettingsView,
    disassembly_view: DisassemblyView,
    breakpoint_view: BreakpointView,
}

pub struct EmulatorState {
    pub run_state: RunState,
    pub last_step_time: Instant,
    pub step_interval: Duration,
}

#[derive(PartialEq)]
pub enum RunState {
    Paused,
    Running,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            last_pc: 0,
            bottom_panel_selected_tab: 0,
            settings_view: SettingsView::default(),
            disassembly_view: DisassemblyView::default(),
            breakpoint_view: BreakpointView::default(),
        }
    }
}

struct BamegoyApp {
    bus: emulator::bus::SharedBus,
    cpu: emulator::cpu::CPU,
    ui_state: UiState,
    emulator_state: EmulatorState,
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

        let b = Rc::new(RefCell::new(Bus::from_cartridge_rom(cartridge_rom).unwrap()));

        Self {
            bus: b.clone(),
            cpu: emulator::cpu::CPU::new(b.clone(), should_trace_log),
            ui_state: UiState::default(),
            emulator_state: EmulatorState {
                run_state: RunState::Paused,
                last_step_time: Instant::now(),
                step_interval: Duration::from_millis(100),
            },
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

        if self.emulator_state.run_state == RunState::Running {

            for bp in &self.ui_state.breakpoint_view.breakpoints {
                if bp.addr == self.cpu.pc {
                    self.emulator_state.run_state = RunState::Paused;
                    return;
                }
            }

            let now = Instant::now();
            if now.duration_since(self.emulator_state.last_step_time) >= self.emulator_state.step_interval {
                self.cpu.step();
                self.emulator_state.last_step_time = now;
            }

            ctx.request_repaint();
        }

        ui::draw_control_panel(ctx, &mut self.cpu, self.bus.clone(), &mut self.ui_state, &mut self.emulator_state);

        egui::SidePanel::left("info_panel").show(ctx, |ui| {
            ui::draw_info_panel(ui, &self.cpu, &mut self.bus);
        });

        egui::TopBottomPanel::bottom("rom_panel")
            .default_height(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui::tabbar::tabbar(ui, &vec!["memory".into(), "serial".into()], &mut self.ui_state.bottom_panel_selected_tab);
                match self.ui_state.bottom_panel_selected_tab {
                    0 => {
                        draw_memory_panel(ui, &self.cpu, &mut self.bus);
                    }
                    1 => {
                        draw_serial_panel(ui, &self.bus.borrow().serial);
                    }
                    _ => unreachable!()
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::disasm::draw_disassembly_panel(ui, &mut self.ui_state, &self.cpu, &mut self.bus);
        });

        ui::settings::draw_settings_window(ctx, &mut self.ui_state.settings_view, &mut self.emulator_state, &mut self.cpu.should_trace_log);

        ui::breakpoints::draw_breakpoint_list_window(ctx, &mut self.ui_state.breakpoint_view);
    }
}
