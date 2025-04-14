use bus::Bus;
use disassemble::disassemble;
use eframe::egui;
use egui::RichText;
use std::{cell::RefCell, env, fs, path::Path, rc::Rc};

pub mod bus;
pub mod cpu;
pub mod disassemble;
pub mod util;

struct UiState {
    disasm_should_scroll: bool,
    disasm_should_follow_pc: bool,
    disasm_scroll_y: f32,
    last_pc: u16,
    current_instruction_index: usize,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            disasm_should_scroll: false,
            disasm_should_follow_pc: true,
            disasm_scroll_y: 0.0,
            last_pc: 0,
            current_instruction_index: 0,
        }
    }
}

struct BamegoyApp {
    bus: bus::SharedBus,
    cpu: cpu::CPU,
    ui_state: UiState,
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
            ui_state: UiState::default(),
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
        egui::TopBottomPanel::top("controls_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("⏵").on_hover_text("Step").clicked() {
                    self.cpu.step();
                }

                ui.button("▶").on_hover_text("Continue");

                ui.button("⏸").on_hover_text("Pause");

                ui.button("⏹").on_hover_text("Stop");

                if ui.button("⟳").on_hover_text("Reset").clicked() {
                    self.cpu = cpu::CPU::new(self.bus.clone());
                }

                // ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                //     ui.label(format!("Loaded ROM: {}", self.rom_path.clone()));
                // })
            });
        });

        egui::SidePanel::left("cpu_info_panel").show(ctx, |ui| {
            ui.heading("CPU");

            ui.monospace(format!("PC:  {:04X}", self.cpu.pc));
            ui.monospace(format!("SP:  {:04X}", self.cpu.sp));

            ui.separator();
            ui.monospace(format!("A:   {:02X}", self.cpu.a));
            ui.monospace(format!(
                "F:   Z={} N={} H={} C={}",
                self.cpu.flags.zero as u8,
                self.cpu.flags.subtraction as u8,
                self.cpu.flags.half_carry as u8,
                self.cpu.flags.carry as u8,
            ));

            ui.separator();
            ui.monospace(format!("B:   {:02X}    C: {:02X}", self.cpu.b, self.cpu.c));
            ui.monospace(format!("D:   {:02X}    E: {:02X}", self.cpu.d, self.cpu.e));
            ui.monospace(format!("H:   {:02X}    L: {:02X}", self.cpu.h, self.cpu.l));

            ui.separator();
            ui.label("current instruction:");
            let (disasm, _) = disassemble(&self.bus.borrow().rom, self.cpu.pc);
            ui.monospace(disasm);
        });

        egui::TopBottomPanel::bottom("rom_panel")
            .default_height(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("ROM Viewer");

                egui::ScrollArea::vertical().show_rows(
                    ui,
                    ui.text_style_height(&egui::TextStyle::Monospace),
                    0x8000 / 16,
                    |ui, row_range| {
                        let rom = &self.bus.borrow().rom;

                        for row in row_range {
                            let addr = row * 16;
                            let chunk = &rom[addr..(addr + 16).min(rom.len())];

                            ui.horizontal(|ui| {
                                ui.monospace(format!("{:04X}:", addr));

                                for (i, byte) in chunk.iter().enumerate() {
                                    let byte_addr = addr + i;

                                    let text = format!("{:02X}", byte);
                                    let rich = if byte_addr == self.cpu.pc as usize {
                                        RichText::new(text).color(egui::Color32::YELLOW)
                                    } else {
                                        RichText::new(text)
                                    };

                                    ui.monospace(rich);
                                }
                            });
                        }
                    },
                );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Disassembly");

            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut self.ui_state.disasm_should_follow_pc,
                    "follow current instruction",
                );
            });

            let rom = &self.bus.borrow().rom;

            let mut pc_lookup = vec![];
            let mut pc = 0x0000;
            let mut instruction_counter = 0;

            while pc < rom.len() {
                pc_lookup.push(pc);
                let (_, size) = disassemble(rom, pc as u16);
                if pc == self.cpu.pc as usize {
                    self.ui_state.current_instruction_index = instruction_counter;
                }
                pc += size as usize;
                instruction_counter += 1;
            }

            let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
            if self.cpu.pc != self.ui_state.last_pc {
                self.ui_state.last_pc = self.cpu.pc;

                if self.ui_state.disasm_should_follow_pc {
                    self.ui_state.disasm_scroll_y = (row_height + 3.0) * self.ui_state.current_instruction_index as f32;
                    self.ui_state.disasm_should_scroll = true;
                }
            }

            let mut scroll_area = egui::ScrollArea::vertical();
            if self.ui_state.disasm_should_scroll {
                scroll_area = scroll_area.vertical_scroll_offset(self.ui_state.disasm_scroll_y);
                self.ui_state.disasm_should_scroll = false;
            }

            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                scroll_area.show_rows(ui, row_height, pc_lookup.len(), |ui, range| {
                    for row in range {
                        let pc = pc_lookup[row];
                        let (instr, _) = disassemble(rom, pc as u16);

                        let text = RichText::new(format!("{:04X}: {}", pc, instr)).monospace();

                        if pc == self.cpu.pc as usize {
                            ui.label(text.background_color(egui::Color32::from_gray(40)));
                        } else {
                            ui.label(text);
                        }
                    }
                });
            });
        });
    }
}
