use std::time::Duration;

use egui::DragValue;

use crate::EmulatorState;


pub struct SettingsView {
    pub show_settings_view: bool,
}

impl Default for SettingsView {
    fn default() -> Self {
        Self {
            show_settings_view: false,
        }
    }
}

pub fn draw_settings_window(ctx: &egui::Context, view: &mut SettingsView, emulator_state: &mut EmulatorState, should_trace: &mut bool) {
    if !view.show_settings_view {
        return;
    }

    egui::Window::new("Settings")
        .default_size(egui::vec2(200.0, 150.0))
        .open(&mut view.show_settings_view)
        .show(ctx, |ui| {
            let mut interval = emulator_state.step_interval.as_millis() as u64;
            ui.horizontal(|ui|{
                ui.label("Step Delay: ");
                ui.add(DragValue::new(&mut interval));
            });
            emulator_state.step_interval = Duration::from_millis(interval);

            ui.checkbox(should_trace, "Trace Log");
        });
}
