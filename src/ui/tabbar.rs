use egui::Ui;

pub fn tabbar(ui: &mut Ui, labels: &Vec<String>, selected: &mut usize) {
    ui.horizontal(|ui| {
        for (idx, label) in labels.iter().enumerate() {
            if ui.button(label).clicked() {
                *selected = idx;
            }
        }
    });
}
