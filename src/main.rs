use bus::Bus;
use disassemble::disassemble;
use eframe::egui;
use egui::RichText;
use std::{cell::RefCell, rc::Rc};

pub mod bus;
pub mod cpu;
pub mod disassemble;
pub mod util;

struct BamegoyApp {
    bus: bus::SharedBus,
    cpu: cpu::CPU,
}

impl Default for BamegoyApp {
    fn default() -> Self {
        let b = Rc::new(RefCell::new(Bus::new()));
        let _ = b.borrow_mut().rom_write_byte(0, 0o04);
        let _ = b.borrow_mut().rom_write_byte(1, 0o14);
        let _ = b.borrow_mut().rom_write_byte(2, 0o24);
        let _ = b.borrow_mut().rom_write_byte(3, 0o34);
        let _ = b.borrow_mut().rom_write_byte(4, 0o44);
        let _ = b.borrow_mut().rom_write_byte(5, 0o54);
        let _ = b.borrow_mut().rom_write_byte(6, 0o74);

        let _ = b.borrow_mut().rom_write_byte(7, 0o05);
        let _ = b.borrow_mut().rom_write_byte(8, 0o15);
        let _ = b.borrow_mut().rom_write_byte(9, 0o25);
        let _ = b.borrow_mut().rom_write_byte(10, 0o35);
        let _ = b.borrow_mut().rom_write_byte(11, 0o45);
        let _ = b.borrow_mut().rom_write_byte(12, 0o55);
        let _ = b.borrow_mut().rom_write_byte(13, 0o75);

        let _ = b.borrow_mut().rom_write_byte(14, 0o06);
        let _ = b.borrow_mut().rom_write_byte(15, 0x69);
        let _ = b.borrow_mut().rom_write_byte(16, 0o16);
        let _ = b.borrow_mut().rom_write_byte(17, 0x69);
        let _ = b.borrow_mut().rom_write_byte(18, 0o26);
        let _ = b.borrow_mut().rom_write_byte(19, 0x69);
        let _ = b.borrow_mut().rom_write_byte(20, 0o36);
        let _ = b.borrow_mut().rom_write_byte(21, 0x69);
        let _ = b.borrow_mut().rom_write_byte(22, 0o46);
        let _ = b.borrow_mut().rom_write_byte(23, 0x69);
        let _ = b.borrow_mut().rom_write_byte(24, 0o56);
        let _ = b.borrow_mut().rom_write_byte(25, 0x69);
        let _ = b.borrow_mut().rom_write_byte(26, 0o76);
        let _ = b.borrow_mut().rom_write_byte(27, 0x69);

        let _ = b.borrow_mut().rom_write_byte(28, 0o303);
        let _ = b.borrow_mut().rom_write_byte(29, 0);
        let _ = b.borrow_mut().rom_write_byte(30, 0);
        Self {
            bus: b.clone(),
            cpu: cpu::CPU::new(b.clone()),
        }
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([680.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "bamegoy",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<BamegoyApp>::default())
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

            let rom = &self.bus.borrow().rom;

            let mut pc_lookup = vec![];
            let mut pc = 0x0000;

            while pc < rom.len() {
                pc_lookup.push(pc);
                let (_, size) = disassemble(rom, pc as u16);
                pc += size as usize;
            }

            let row_height = ui.text_style_height(&egui::TextStyle::Monospace);

            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    pc_lookup.len(),
                    |ui, range| {
                        for row in range {
                            let pc = pc_lookup[row];
                            let (instr, _) = disassemble(rom, pc as u16);

                            let label = if pc == self.cpu.pc as usize {
                                RichText::new(format!("{:04X}: {}", pc, instr))
                                    .monospace()
                                    .background_color(egui::Color32::from_gray(45))
                            } else {
                                RichText::new(format!("{:04X}: {}", pc, instr)).monospace()
                            };

                            ui.label(label);
                        }
                    },
                );
            });
        });
    }
}
