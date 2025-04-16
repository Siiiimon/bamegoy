use bus::Bus;
use disassemble::disassemble;
use eframe::egui;
use ui::draw_memory_panel;
use std::{cell::RefCell, env, fs, path::Path, rc::Rc};

pub mod bus;
pub mod cpu;
pub mod instruction;
pub mod disassemble;
pub mod util;

mod ui;

struct BamegoyApp {
    bus: bus::SharedBus,
    cpu: cpu::CPU,
    ui_state: ui::UiState,
}

impl BamegoyApp {
    pub fn new(rom_filepath: Option<&Path>) -> Self {
        let cartridge_rom: Vec<u8> = match rom_filepath {
            Some(p) => match fs::read(p) {
                Err(e) => {
                    eprintln!("failed to read {:?}: {}", p, e);
                    vec![0; 0x8000]
                }
                Ok(c) => c,
            },
            None => vec![0; 0x8000],
        };

        let b = Rc::new(RefCell::new(Bus::new()));
        match b.borrow_mut().from_cartridge_rom(cartridge_rom) {
            Err(e) => {
                eprintln!("{}", e);
                ()
            }
            Ok(_) => (),
        }

        Self {
            bus: b.clone(),
            cpu: cpu::CPU::new(b.clone()),
            ui_state: ui::UiState::default(),
        }
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let rom_filepath: Option<&Path> = match args.get(1) {
        Some(p) => Some(Path::new(p)),
        None => None,
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([680.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "bamegoy",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(BamegoyApp::new(rom_filepath)))
        }),
    )
}

impl eframe::App for BamegoyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        ui::draw_control_panel(ctx, &mut self.cpu, self.bus.clone());

        egui::SidePanel::left("info_panel").show(ctx, |ui| {
            ui::draw_info_panel(ui, &self.cpu, &mut self.bus);
        });

        egui::TopBottomPanel::bottom("rom_panel")
            .default_height(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                draw_memory_panel(ui, &self.cpu, &mut self.bus);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::draw_disassembly_panel(ui, &mut self.ui_state, &self.cpu, &mut self.bus);
        });
    }
}
