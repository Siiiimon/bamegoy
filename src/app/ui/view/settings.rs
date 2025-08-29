use bamegoy::emulator::cpu::CPU;

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

pub fn draw_settings_view(ctx: &egui::Context, view: &mut SettingsView, cpu: &mut CPU) {
    // if !view.show_settings_view {
    //     return;
    // }

    // egui::Window::new("Settings")
    //     .default_size(egui::vec2(200.0, 150.0))
    //     .open(&mut view.show_settings_view)
    //     .show(ctx, |ui| {
    //         ui.checkbox(&mut cpu.should_trace_log, "Trace Log");
    //     });
}
