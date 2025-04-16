use egui::RichText;

use crate::{UiState, bus::SharedBus, bus::io::serial, cpu::CPU, disassemble};

pub mod tabbar;

pub fn draw_control_panel(ctx: &egui::Context, cpu: &mut CPU, bus: SharedBus) {
    egui::TopBottomPanel::top("controls_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("⏵").on_hover_text("Step").clicked() {
                cpu.step();
            }

            ui.button("▶").on_hover_text("Continue");

            ui.button("⏸").on_hover_text("Pause");

            ui.button("⏹").on_hover_text("Stop");

            if ui.button("⟳").on_hover_text("Reset").clicked() {
                cpu.reset(bus);
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
    let (mnemonic, _) = disassemble(&bus.borrow().rom, cpu.pc);
    ui.monospace(mnemonic);
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

pub fn draw_disassembly_panel(
    ui: &mut egui::Ui,
    ui_state: &mut UiState,
    cpu: &CPU,
    bus: &mut SharedBus,
) {
    ui.heading("Disassembly");

    ui.horizontal(|ui| {
        ui.checkbox(
            &mut ui_state.disasm_should_follow_pc,
            "follow current instruction",
        );
    });

    let rom = &bus.borrow().rom;

    let mut pc_lookup = vec![];
    let mut pc = 0x0000;
    let mut instruction_counter = 0;

    while pc < rom.len() {
        pc_lookup.push(pc);
        let (_, size) = disassemble(rom, pc as u16);
        if pc == cpu.pc as usize {
            ui_state.current_instruction_index = instruction_counter;
        }
        pc += size as usize;
        instruction_counter += 1;
    }

    let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
    if cpu.pc != ui_state.last_pc {
        ui_state.last_pc = cpu.pc;

        if ui_state.disasm_should_follow_pc {
            ui_state.disasm_scroll_y =
                (row_height + 3.0) * ui_state.current_instruction_index as f32;
            ui_state.disasm_should_scroll = true;
        }
    }

    let mut scroll_area = egui::ScrollArea::vertical();
    if ui_state.disasm_should_scroll {
        scroll_area = scroll_area.vertical_scroll_offset(ui_state.disasm_scroll_y);
        ui_state.disasm_should_scroll = false;
    }

    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
        scroll_area.show_rows(ui, row_height, pc_lookup.len(), |ui, range| {
            for row in range {
                let pc = pc_lookup[row];
                let (instr, _) = disassemble(rom, pc as u16);

                let text = RichText::new(format!("{:04X}: {}", pc, instr)).monospace();

                if pc == cpu.pc as usize {
                    ui.label(text.background_color(egui::Color32::from_gray(40)));
                } else {
                    ui.label(text);
                }
            }
        });
    });
}

pub fn draw_serial_panel(ui: &mut egui::Ui, serial: &serial::Serial) {
    ui.horizontal(|ui| {
        ui.label(format!(
            "SC Transfer {}",
            if serial.control.enable {
                "enabled"
            } else {
                "disabled"
            }
        ));
        ui.spacing();
        ui.label(format!(
            "Using {} Clock",
            if serial.control.should_use_internal_clock {
                "internal"
            } else {
                "external"
            }
        ));
    });
    ui.label(format!("SB Register: 0x{:02X}", serial.content));
    ui.separator();
}
