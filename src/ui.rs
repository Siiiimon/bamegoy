use egui::{Context, RichText};
use std::sync::mpsc::Sender;

use bamegoy::emulator::bus::Bus;
use bamegoy::emulator::cpu::CPU;
use bamegoy::emulator::{DriverMessage, Handle};
use bamegoy::emulator::bus::io::serial;
// use crate::emulator::disassemble::disassemble;
use crate::UiState;

pub mod settings;
pub mod tabbar;
pub mod breakpoints;
pub mod disasm;
mod control_panel;

pub fn draw_error(ctx: &Context, msg: String) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(format!("error: {}", msg));
    });
}

pub fn draw_running(ctx: &Context, handle: &mut Handle) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("emulator running, data unavailable.");
        if ui.button("Pause").clicked() {
            handle.tx.send(DriverMessage::PauseRequest).unwrap();
        }
    });
}

pub fn draw(ctx: &Context, state: &mut UiState, tx: Sender<DriverMessage>, bus: &mut Bus, cpu: &mut CPU) {
    control_panel::draw(ctx, cpu, state, tx);

    egui::SidePanel::left("info_panel").show(ctx, |ui| {
        draw_info_panel(ui, &cpu, bus);
    });

    egui::TopBottomPanel::bottom("rom_panel")
        .default_height(250.0)
        .resizable(true)
        .show(ctx, |ui| {
            tabbar::tabbar(ui, &vec!["memory".into(), "serial".into()], &mut state.bottom_panel_selected_tab);
            match state.bottom_panel_selected_tab {
                0 => {
                    draw_memory_panel(ui, &cpu, bus);
                }
                1 => {
                    draw_serial_panel(ui, &bus.serial);
                }
                _ => unreachable!()
            }
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        disasm::draw_disassembly_panel(ui, state, &cpu, bus);
    });

    settings::draw_settings_window(ctx, &mut state.settings_view, cpu);

    breakpoints::draw_breakpoint_list_window(ctx, &mut state.breakpoint_view);
}


pub fn draw_info_panel(ui: &mut egui::Ui, cpu: &CPU, bus: &mut Bus) {
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
    // if let Some(disasm) = disassemble(&bus, cpu.pc) {
    //     ui.monospace(format!("mnemonic: {}", disasm.mnemonic));
    //     ui.monospace(format!(
    //         "bytes: {}",
    //         disasm
    //             .bytes
    //             .iter()
    //             .map(|b| format!("{:02X}", b))
    //             .collect::<Vec<_>>()
    //             .join(" ")
    //     ));
    // }
}

pub fn draw_memory_panel(ui: &mut egui::Ui, cpu: &CPU, bus: &mut Bus) {
    ui.heading("Memory");

    egui::ScrollArea::vertical().show_rows(
        ui,
        ui.text_style_height(&egui::TextStyle::Monospace),
        0x8000 / 16,
        |ui, row_range| {
            let rom = &bus.rom;

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