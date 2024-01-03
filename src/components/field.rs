pub struct Field {
    title: String,
    value: String,
}

impl Field {
    pub fn new(title: &str, value: &str) -> Self {
        Self {
            title: String::from(title),
            value: String::from(value)
        }
    }
}

impl egui::Widget for Field {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let title_size = egui::Vec2::new(ui.available_width() / 3.0 - 8.0, 20.0);
        let title = egui::RichText::new(self.title.clone())
            .size(12.0)
            .color(egui::Color32::GRAY);
        let value = egui::RichText::new(self.value.clone())
            .size(12.0)
            .color(egui::Color32::WHITE);
        ui.set_min_width(ui.available_width() - 8.0);
        ui.horizontal(|ui| {
            let response = ui.allocate_ui_with_layout(title_size,
                egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Label::new(title))
                }).inner;
            response | ui.add(egui::Label::new(value))
        }).inner
    }
}
