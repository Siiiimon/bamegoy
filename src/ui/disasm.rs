use egui::RichText;

use crate::{bus::SharedBus, cpu::CPU, disassemble, UiState};

pub struct DisassemblyView {
    pub disasm_should_follow_pc: bool,
    pub current_instruction_index: u16,
    pub should_scroll: bool,
    pub scroll_y: f32,
}

impl Default for DisassemblyView {
    fn default() -> Self {
        Self {
            disasm_should_follow_pc: true,
            current_instruction_index: 0,
            should_scroll: true,
            scroll_y: 0.0,
        }
    }
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
            &mut ui_state.disassembly_view.disasm_should_follow_pc,
            "follow current instruction",
        );
    });

    let mut pc_lookup = vec![];
    let mut pc = 0x0000;
    let mut instruction_counter = 0;

    while pc < bus.borrow().rom.len() {
        pc_lookup.push(pc);
        let disasm = disassemble(&bus.borrow(), pc as u16).unwrap();
        if pc == cpu.pc as usize {
            ui_state.disassembly_view.current_instruction_index = instruction_counter;
        }
        pc += disasm.length as usize;
        instruction_counter += 1;
    }

    let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
    if cpu.pc != ui_state.last_pc {
        ui_state.last_pc = cpu.pc;

        if ui_state.disassembly_view.disasm_should_follow_pc {
            ui_state.disassembly_view.scroll_y =
                (row_height + 3.0) * ui_state.disassembly_view.current_instruction_index as f32;
            ui_state.disassembly_view.should_scroll = true;
        }
    }

    let mut scroll_area = egui::ScrollArea::vertical();
    if ui_state.disassembly_view.should_scroll {
        scroll_area = scroll_area.vertical_scroll_offset(ui_state.disassembly_view.scroll_y);
        ui_state.disassembly_view.should_scroll = false;
    }

    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
        scroll_area.show_rows(ui, row_height, pc_lookup.len(), |ui, range| {
            for row in range {
                let pc = pc_lookup[row];
                let disasm = disassemble(&bus.borrow(), pc as u16).unwrap();

                let text = RichText::new(format!("{:04X}: {}", pc, disasm.mnemonic)).monospace();

                if pc == cpu.pc as usize {
                    ui.label(text.background_color(egui::Color32::from_gray(40)));
                } else {
                    ui.label(text);
                }
            }
        });
    });
}
