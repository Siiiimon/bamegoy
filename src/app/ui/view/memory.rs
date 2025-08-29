pub struct MemoryView {}

impl Default for MemoryView {
    fn default() -> Self {
        Self {  }
    }
}

pub fn draw_memory_view(ui: &mut egui::Ui) {
    // ui.heading("Memory");

    // egui::ScrollArea::vertical().show_rows(
    //     ui,
    //     ui.text_style_height(&egui::TextStyle::Monospace),
    //     0x8000 / 16,
    //     |ui, row_range| {
    //         let rom = &bus.rom;

    //         for row in row_range {
    //             let addr = row * 16;
    //             let chunk = &rom[addr..(addr + 16).min(rom.len())];

    //             ui.horizontal(|ui| {
    //                 ui.monospace(format!("{:04X}:", addr));

    //                 for (i, byte) in chunk.iter().enumerate() {
    //                     let byte_addr = addr + i;

    //                     let text = format!("{:02X}", byte);
    //                     let rich = if byte_addr == cpu.pc as usize {
    //                         RichText::new(text).color(egui::Color32::YELLOW)
    //                     } else {
    //                         RichText::new(text)
    //                     };

    //                     ui.monospace(rich);
    //                 }
    //             });
    //         }
    //     },
    // );
}