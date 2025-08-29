pub struct InfoView {}

impl Default for InfoView {
    fn default() -> Self {
        Self {  }
    }
}

pub fn draw_info_view(ui: &mut egui::Ui) {
    // ui.heading("CPU");

    // ui.monospace(format!("PC:  {:04X}", cpu.pc));
    // ui.monospace(format!("SP:  {:04X}", cpu.sp));

    // ui.separator();
    // ui.monospace(format!("A:   {:02X}", cpu.a));
    // ui.monospace(format!(
    //     "F:   Z={} N={} H={} C={}",
    //     cpu.flags.zero as u8,
    //     cpu.flags.subtraction as u8,
    //     cpu.flags.half_carry as u8,
    //     cpu.flags.carry as u8,
    // ));

    // ui.separator();
    // ui.monospace(format!("B:   {:02X}    C: {:02X}", cpu.b, cpu.c));
    // ui.monospace(format!("D:   {:02X}    E: {:02X}", cpu.d, cpu.e));
    // ui.monospace(format!("H:   {:02X}    L: {:02X}", cpu.h, cpu.l));

    // ui.separator();
    // ui.label("current instruction:");
    // if let Some(disasm) = disassemble(&bus, cpu.pc) {
    //     ui.monospace(format!("mnemonic: {}", disasm.mnemonic));
    //     ui.monospace(format!(
    //         "bytes: {}",
    //         disasm
    //             .bytes
    //             .iter()
    //             .map(|b| format!("{:02X}", b))
    //             .collect::<Vec<_>>()
    //             .join(" ")
    //     ));
    // }
}