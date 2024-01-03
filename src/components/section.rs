struct SectionToggle;

impl SectionToggle {
    const WIDTH: f32 = 30.0;
    const HEIGHT: f32 = 8.0;
    const SIZE: egui::Vec2 = egui::Vec2::new(Self::WIDTH, Self::HEIGHT);
    const RADIUS: f32 = 4.0;
}

impl egui::Widget for SectionToggle {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) = ui
            .allocate_painter(Self::SIZE, egui::Sense::click());
        let bounds = egui::Rect::from_min_size(response.rect.min, Self::SIZE);
        painter
            .rect_filled(bounds, Self::RADIUS, egui::Color32::DARK_GRAY);
        response
    }
}

pub struct SectionTitle(pub String);

impl egui::Widget for SectionTitle {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let title = egui::RichText::new(self.0)
            .size(15.0)
            .color(egui::Color32::WHITE);
        ui.label(title)
    }
}

pub struct Section {
    title: String,
    is_shown: bool,
}

impl Section {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            is_shown: true,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui,
        content: impl FnOnce(&mut egui::Ui) -> egui::Response) -> egui::Response {
        egui::Frame::none()
            .fill(egui::Color32::from_gray(49))
            .rounding(4.0)
            .inner_margin(12.0)
            .outer_margin(12.0)
            .show(ui, |ui| {
                ui.set_width(250.0);
                let mut response = ui.add(SectionTitle(self.title.clone()));
                if self.is_shown {
                    response |= content(ui);
                }
                ui.vertical_centered(|ui| {
                    if ui.add(SectionToggle).clicked() {
                        self.is_shown = !self.is_shown;
                    }
                });
                response
            }).inner
    }
}
