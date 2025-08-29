use egui::{Color32, RichText};

pub struct BreakpointView {
    pub breakpoints: Vec<Breakpoint>,
    pub show_breakpoint_view: bool,
    new_breakpoint_addr: String,
}

impl Default for BreakpointView {
    fn default() -> Self {
        Self {
            breakpoints: vec![],
            show_breakpoint_view: false,
            new_breakpoint_addr: "".into(),
        }
    }
}

pub struct Breakpoint {
    pub addr: u16,
    pub is_active: bool,
}

impl Breakpoint {
    pub fn new(addr: u16) -> Self {
        Self {
            addr,
            is_active: true,
        }
    }
}

pub fn draw_breakpoint_list_view(ctx: &egui::Context, view: &mut BreakpointView) {
    // if !view.show_breakpoint_view {
    //     return;
    // }
    // egui::Window::new("Breakpoints")
    //     .default_size(egui::vec2(200.0, 150.0))
    //     .open(&mut view.show_breakpoint_view)
    //     .show(ctx, |ui| {
    //         ui.vertical(|ui| {
    //             let mut to_remove = vec![];

    //             for (i, bp) in view.breakpoints.iter().enumerate() {
    //                 ui.horizontal(|ui| {
    //                     if bp.is_active {
    //                         ui.label(
    //                             RichText::new(format!("0x{:04X}", bp.addr))
    //                                 .color(Color32::from_rgb(255, 212, 71)),
    //                         );
    //                     } else {
    //                         ui.label(RichText::new(format!("0x{:04X}", bp.addr)).weak());
    //                     }
    //                     if ui.button("x").on_hover_text("Remove breakpoint").clicked() {
    //                         to_remove.push(i);
    //                     }
    //                 });
    //             }

    //             for i in to_remove {
    //                 view.breakpoints.remove(i);
    //             }

    //             ui.separator();

    //             ui.horizontal(|ui| {
    //                 ui.label("New:");
    //                 let response = ui.text_edit_singleline(&mut view.new_breakpoint_addr);

    //                 if ui.button("Add").clicked()
    //                     || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
    //                 {
    //                     if let Ok(addr) = u16::from_str_radix(
    //                         &view.new_breakpoint_addr.trim_start_matches("0x"),
    //                         16,
    //                     ) {
    //                         view.breakpoints.push(Breakpoint::new(addr));
    //                     }
    //                     view.new_breakpoint_addr.clear();
    //                 }
    //             });
    //         });
    //     });
}
