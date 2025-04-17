use std::time::Instant;

use egui::RichText;

use crate::{
    EmulatorState, RunState, UiState,
    bus::{SharedBus, io::serial},
    cpu::CPU,
    disassemble,
};

pub mod settings;
pub mod tabbar;
pub mod breakpoints;
pub mod disasm;

pub fn draw_control_panel(
    ctx: &egui::Context,
    cpu: &mut CPU,
    bus: SharedBus,
    ui_state: &mut UiState,
    emulator_state: &mut EmulatorState,
) {
    egui::TopBottomPanel::top("controls_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Settings").clicked() {
                ui_state.settings_view.show_settings_view = true;
            }
        });
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("⏵").on_hover_text("Step").clicked() {
                cpu.step();
            }

            if emulator_state.run_state == RunState::Running {
                if ui.button("⏸").on_hover_text("Pause").clicked() {
                    emulator_state.run_state = RunState::Paused;
                }
            } else {
                if ui.button("▶").on_hover_text("Continue").clicked() {
                    emulator_state.run_state = RunState::Running;
                    emulator_state.last_step_time = Instant::now();
                }
            }

            if ui.button("⟳").on_hover_text("Reset").clicked() {
                cpu.reset(bus);
            }

            if ui.button("B").on_hover_text("Breakpoints").clicked() {
                ui_state.breakpoint_view.show_breakpoint_view = !ui_state.breakpoint_view.show_breakpoint_view;
            }

            // ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            //     ui.label(format!("Loaded ROM: {}", self.rom_path.clone()));
            // })
        });
    });
}

pub fn draw_info_panel(ui: &mut egui::Ui, cpu: &CPU, bus: &mut SharedBus) {
    ui.heading("CPU");

    ui.monospace(format!("PC:  {:04X}", cpu.pc));
    ui.monospace(format!("SP:  {:04X}", cpu.sp));

    ui.separator();
    ui.monospace(format!("A:   {:02X}", cpu.a));
    ui.monospace(format!(
        "F:   Z={} N={} H={} C={}",
        cpu.flags.zero as u8,
        cpu.flags.subtraction as u8,
        cpu.flags.half_carry as u8,
        cpu.flags.carry as u8,
    ));

    ui.separator();
    ui.monospace(format!("B:   {:02X}    C: {:02X}", cpu.b, cpu.c));
    ui.monospace(format!("D:   {:02X}    E: {:02X}", cpu.d, cpu.e));
    ui.monospace(format!("H:   {:02X}    L: {:02X}", cpu.h, cpu.l));

    ui.separator();
    ui.label("current instruction:");
    if let Some(disasm) = disassemble(&bus.borrow(), cpu.pc) {
        ui.monospace(format!("mnemonic: {}", disasm.mnemonic));
        ui.monospace(format!(
            "bytes: {}",
            disasm
                .bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }
}

pub fn draw_memory_panel(ui: &mut egui::Ui, cpu: &CPU, bus: &mut SharedBus) {
    ui.heading("Memory");

    egui::ScrollArea::vertical().show_rows(
        ui,
        ui.text_style_height(&egui::TextStyle::Monospace),
        0x8000 / 16,
        |ui, row_range| {
            let rom = &bus.borrow().rom;

            for row in row_range {
                let addr = row * 16;
                let chunk = &rom[addr..(addr + 16).min(rom.len())];

                ui.horizontal(|ui| {
                    ui.monospace(format!("{:04X}:", addr));

                    for (i, byte) in chunk.iter().enumerate() {
                        let byte_addr = addr + i;

                        let text = format!("{:02X}", byte);
                        let rich = if byte_addr == cpu.pc as usize {
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
}

pub fn draw_serial_panel(ui: &mut egui::Ui, serial: &serial::Serial) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(format!(
                "SC Transfer: {}",
                if serial.control.enable {
                    "Enabled"
                } else {
                    "Disabled"
                }
            ));
            ui.separator();
            ui.label(format!(
                "Clock: {}",
                if serial.control.should_use_internal_clock {
                    "Internal"
                } else {
                    "External"
                }
            ));
        });

        ui.horizontal(|ui| {
            ui.label(format!(
                "SB Register: 0x{:02X} ('{}')",
                serial.content,
                match serial.content {
                    0x20..=0x7E => serial.content as char,
                    _ => '.',
                }
            ));
            ui.separator();
            ui.label(format!("Total sent: {}", serial.outgoing.len()));
        })
    });

    ui.separator();
    ui.label("Sent Bytes:");

    let hex = serial
        .outgoing
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ");

    let ascii = serial
        .outgoing
        .iter()
        .map(|b| {
            let c = *b as char;
            if c.is_ascii_graphic() { c } else { '.' }
        })
        .collect::<String>();

    ui.monospace(format!("Hex:   {}", hex));
    ui.monospace(format!("ASCII: {}", ascii));
}

