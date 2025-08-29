pub struct SerialView {}

impl Default for SerialView {
    fn default() -> Self {
        Self {  }
    }
}

pub fn draw_serial_view(ui: &mut egui::Ui) {
    // ui.group(|ui| {
    //     ui.horizontal(|ui| {
    //         ui.label(format!(
    //             "SC Transfer: {}",
    //             if serial.control.enable {
    //                 "Enabled"
    //             } else {
    //                 "Disabled"
    //             }
    //         ));
    //         ui.separator();
    //         ui.label(format!(
    //             "Clock: {}",
    //             if serial.control.should_use_internal_clock {
    //                 "Internal"
    //             } else {
    //                 "External"
    //             }
    //         ));
    //     });

    //     ui.horizontal(|ui| {
    //         ui.label(format!(
    //             "SB Register: 0x{:02X} ('{}')",
    //             serial.content,
    //             match serial.content {
    //                 0x20..=0x7E => serial.content as char,
    //                 _ => '.',
    //             }
    //         ));
    //         ui.separator();
    //         ui.label(format!("Total sent: {}", serial.outgoing.len()));
    //     })
    // });

    // ui.separator();
    // ui.label("Sent Bytes:");

    // let hex = serial
    //     .outgoing
    //     .iter()
    //     .map(|b| format!("{:02X}", b))
    //     .collect::<Vec<_>>()
    //     .join(" ");

    // let ascii = serial
    //     .outgoing
    //     .iter()
    //     .map(|b| {
    //         let c = *b as char;
    //         if c.is_ascii_graphic() { c } else { '.' }
    //     })
    //     .collect::<String>();

    // ui.monospace(format!("Hex:   {}", hex));
    // ui.monospace(format!("ASCII: {}", ascii));
}