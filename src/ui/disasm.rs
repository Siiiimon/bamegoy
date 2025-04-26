use crate::emulator::disassemble::{disassemble, Operand};
use crate::emulator::{disassemble, util::color32_from_catppuccin_with_alpha};
use egui::{Color32, FontId, TextFormat, text::LayoutJob};
use crate::{COLORS, UiState};
use crate::emulator::{bus::Bus, cpu::CPU, util::color32_from_catppuccin};

use super::breakpoints::Breakpoint;

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
    bus: &mut Bus,
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

    while pc < bus.rom.len() {
        pc_lookup.push(pc);
        let disasm = disassemble(&bus, pc as u16)
            .expect(&format!("No opcode byte at address {:04X}", pc));
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
            let painter = ui.painter();
            painter.rect_filled(ui.max_rect(), 0.0, color32_from_catppuccin(COLORS.mantle));

            for row in range {
                let pc = pc_lookup[row];
                let disasm = disassemble(&bus, pc as u16)
                    .expect(&format!("No opcode byte at address {:04X}", pc));
                let is_active = pc == cpu.pc as usize;

                let layout_job = format_mnemonic(pc as u16, disasm, is_active);

                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), row_height),
                    egui::Sense::hover(),
                );

                let response = ui.interact(rect, ui.make_persistent_id(pc), egui::Sense::click());

                response.context_menu(|ui| {
                    if let Some(pos) = ui_state
                        .breakpoint_view
                        .breakpoints
                        .iter()
                        .position(|b| b.addr == pc as u16)
                    {
                        if ui.button("Clear breakpoint").clicked() {
                            ui_state.breakpoint_view.breakpoints.remove(pos);
                            ui.close_menu();
                        }
                    } else {
                        if ui.button("Set breakpoint").clicked() {
                            ui_state.breakpoint_view.breakpoints.push(Breakpoint {
                                addr: pc as u16,
                                is_active: true,
                            });
                            ui.close_menu();
                        }
                    }
                });

                let painter = ui.painter();
                let bg_color = if is_active {
                    color32_from_catppuccin(COLORS.base)
                } else {
                    if ui_state
                        .breakpoint_view
                        .breakpoints
                        .iter()
                        .any(|b| b.addr == pc as u16)
                    {
                        color32_from_catppuccin_with_alpha(COLORS.red, 70)
                    } else {
                        color32_from_catppuccin(COLORS.mantle)
                    }
                };
                painter.rect_filled(rect, 0.0, bg_color);

                let galley = ui.fonts(|f| f.layout_job(layout_job));
                painter.galley(rect.left_top(), galley, Color32::DEBUG_COLOR);
            }
        });
    });
}

fn format_mnemonic(addr: u16, disasm: disassemble::Disasm, is_current: bool) -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append(
        &format!("{:04X}", addr),
        0.0,
        TextFormat {
            font_id: FontId::monospace(12.0),
            color: if is_current {
                color32_from_catppuccin(COLORS.lavender)
            } else {
                color32_from_catppuccin(COLORS.overlay1)
            },
            ..Default::default()
        },
    );

    job.append(
        &disasm.verb,
        18.0,
        TextFormat {
            font_id: FontId::monospace(12.0),
            color: color32_from_catppuccin(COLORS.mauve),
            ..Default::default()
        },
    );

    for operand in disasm.operands {
        let operand_color = match operand {
            Operand::Register8(_) | Operand::Register16(_) => color32_from_catppuccin(COLORS.green),
            Operand::Immediate8(_) | Operand::Immediate16(_) => {
                color32_from_catppuccin(COLORS.blue)
            }
            Operand::Address(_) | Operand::Offset(_) | Operand::MemoryIndirect(_) => {
                color32_from_catppuccin(COLORS.peach)
            }
            Operand::Conditional(_) => color32_from_catppuccin(COLORS.rosewater),
            Operand::Raw(_) => color32_from_catppuccin(COLORS.subtext0),
        };
        job.append(
            &format!("{operand}"),
            10.0,
            TextFormat {
                font_id: FontId::monospace(12.0),
                color: operand_color,
                ..Default::default()
            },
        );
    }

    job
}
