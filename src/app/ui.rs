use egui::Context;
use std::sync::mpsc::Sender;

use bamegoy::emulator::DriverMessage;

use crate::app::AppState;

pub mod view;
pub mod components;

// pub fn draw_error(ctx: &Context, msg: String) {
//     // egui::CentralPanel::default().show(ctx, |ui| {
//     //     ui.label(format!("error: {}", msg));
//     // });
// }

// pub fn draw_running(ctx: &Context, handle: &mut Handle) {
//     // egui::CentralPanel::default().show(ctx, |ui| {
//     //     ui.label("emulator running, data unavailable.");
//     //     if ui.button("Pause").clicked() {
//     //         handle.tx.send(DriverMessage::PauseRequest).unwrap();
//     //     }
//     // });
// }

pub fn draw(ctx: &Context, state: &mut AppState, tx: Sender<DriverMessage>) {
    // view::controlpanel::draw_controlpanel_view(ctx, state, tx);

    // egui::SidePanel::left("info_panel").show(ctx, |ui| {
    //     draw_info_panel(ui, &cpu, bus);
    // });

    // egui::TopBottomPanel::bottom("rom_panel")
    //     .default_height(250.0)
    //     .resizable(true)
    //     .show(ctx, |ui| {
    //         tabbar::tabbar(ui, &vec!["memory".into(), "serial".into()], &mut state.bottom_panel_selected_tab);
    //         match state.bottom_panel_selected_tab {
    //             0 => {
    //                 draw_memory_panel(ui, &cpu, bus);
    //             }
    //             1 => {
    //                 draw_serial_panel(ui, &bus.serial);
    //             }
    //             _ => unreachable!()
    //         }
    //     });

    // egui::CentralPanel::default().show(ctx, |ui| {
    //     disasm::draw_disassembly_panel(ui, state, &cpu, bus);
    // });

    // settings::draw_settings_window(ctx, &mut state.settings_view, cpu);

    // breakpoints::draw_breakpoint_list_window(ctx, &mut state.breakpoint_view);
}