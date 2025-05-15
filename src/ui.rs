use egui::{Context, RichText};
use std::sync::mpsc::Sender;

use crate::emulator::bus::{Bus, BusView};
use crate::emulator::cpu::{CpuView, CPU};
use crate::emulator::{DriverMessage, Handle};
use crate::emulator::bus::io::serial;
use crate::emulator::disassemble::disassemble;
use crate::emulator::util::{Register, RegisterPair};
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

pub fn draw(ctx: &Context, state: &mut UiState, tx: Sender<DriverMessage>, bus: &mut Box<dyn BusView>, cpu: &mut Box<dyn CpuView>) {
    control_panel::draw(ctx, state, tx);

    egui::SidePanel::left("info_panel").show(ctx, |ui| {
        draw_info_panel(ui, cpu.clone(), bus);
    });

    egui::TopBottomPanel::bottom("rom_panel")
        .default_height(250.0)
        .resizable(true)
        .show(ctx, |ui| {
            tabbar::tabbar(ui, &vec!["memory".into(), "serial".into()], &mut state.bottom_panel_selected_tab);
            match state.bottom_panel_selected_tab {
                0 => {
                    draw_memory_panel(ui, cpu.clone(), bus);
                }
                1 => {
                    draw_serial_panel(ui, bus.get_serial());
                }
                _ => unreachable!()
            }
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        disasm::draw_disassembly_panel(ui, state, cpu.clone(), bus.clone());
    });

    settings::draw_settings_window(ctx, &mut state.settings_view, cpu.clone());

    breakpoints::draw_breakpoint_list_window(ctx, &mut state.breakpoint_view);
}


pub fn draw_info_panel(ui: &mut egui::Ui, cpu: Box<dyn CpuView>, bus: &mut Box<dyn BusView>) {
    let flags = cpu.get_flags();
    ui.heading("CPU");

    ui.monospace(format!("PC:  {:04X}", cpu.get_pc()));
    ui.monospace(format!("SP:  {:04X}", cpu.get_register_pair(RegisterPair::SP)));

    ui.separator();
    ui.monospace(format!("A:   {:02X}", cpu.get_register(Register::A)));
    ui.monospace(format!(
        "F:   Z={} N={} H={} C={}",
        flags.zero as u8,
        flags.subtraction as u8,
        flags.half_carry as u8,
        flags.carry as u8,
    ));

    ui.separator();
    ui.monospace(format!("B:   {:02X}    C: {:02X}", cpu.get_register(Register::B), cpu.get_register(Register::C)));
    ui.monospace(format!("D:   {:02X}    E: {:02X}", cpu.get_register(Register::D), cpu.get_register(Register::E)));
    ui.monospace(format!("H:   {:02X}    L: {:02X}", cpu.get_register(Register::H), cpu.get_register(Register::L)));

    ui.separator();
    ui.label("current instruction:");
    if let Some(disasm) = disassemble(bus.clone(), cpu.get_pc()) {
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

pub fn draw_memory_panel(ui: &mut egui::Ui, cpu: Box<dyn CpuView>, bus: Box<dyn BusView>) {
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
                        let rich = if byte_addr == cpu.get_pc() as usize {
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