use crate::emulator::cpu::CpuView;
use crate::emulator::{policy, DriverMessage, State};
use crate::UiState;
use std::sync::mpsc::Sender;

pub fn draw(
    ctx: &egui::Context,
    state: &mut UiState,
    tx: Sender<DriverMessage>,
) {
    egui::TopBottomPanel::top("controls_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Settings").clicked() {
                state.settings_view.show_settings_view = true;
            }
        });
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("⏵").on_hover_text("Step").clicked() {
                tx.send(DriverMessage::Run(Some(policy::single_step()))).unwrap();
            }

            if state.emulator_state == State::Running {
                if ui.button("⏸").on_hover_text("Pause").clicked() {
                    tx.send(DriverMessage::PauseRequest).unwrap();
                }
            } else {
                if ui.button("▶").on_hover_text("Continue").clicked() {
                    tx.send(DriverMessage::Run(None)).unwrap();
                }
            }

            // if ui.button("⟳").on_hover_text("Reset").clicked() {
            //     cpu.reset(bus);
            // }

            if ui.button("B").on_hover_text("Breakpoints").clicked() {
                state.breakpoint_view.show_breakpoint_view = !state.breakpoint_view.show_breakpoint_view;
            }

            // ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            //     ui.label(format!("Loaded ROM: {}", self.rom_path.clone()));
            // })
        });
    });
}