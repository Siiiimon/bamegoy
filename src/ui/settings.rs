use crate::emulator::cpu::{CpuView, CPU};

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

pub fn draw_settings_window(ctx: &egui::Context, view: &mut SettingsView, mut cpu: Box<dyn CpuView>) {
    if !view.show_settings_view {
        return;
    }

    egui::Window::new("Settings")
        .default_size(egui::vec2(200.0, 150.0))
        .open(&mut view.show_settings_view)
        .show(ctx, |ui| {
            if let Some(cpu) = cpu.as_any_mut().downcast_mut::<CPU>() {
                ui.checkbox(&mut cpu.should_trace_log, "Trace Log");
            } else {
                ui.add_enabled(false, egui::Checkbox::new(&mut false, "Trace Log"));
            }
        });
}
