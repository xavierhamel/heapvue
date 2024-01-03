pub struct PlayToggle(pub bool);

impl PlayToggle {
    const WIDTH: f32 = 40.0;
    const HEIGHT: f32 = 40.0;
    const SIZE: egui::Vec2 = egui::Vec2::new(Self::WIDTH, Self::HEIGHT);
    const MARGIN: egui::Vec2 = egui::Vec2::new(5.0, 5.0);
    const RADIUS: f32 = 3.0;
}

impl egui::Widget for PlayToggle {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) = ui
            .allocate_painter(Self::SIZE, egui::Sense::click());
        let bounds = egui::Rect::from_min_size(response.rect.min, Self::SIZE);
        painter
            .rect_filled(bounds, Self::RADIUS, egui::Color32::DARK_GRAY);
        let inner_bounds = egui::Rect::from_min_size(response.rect.min + Self::MARGIN,
            Self::SIZE - Self::MARGIN);
        painter
            .rect_filled(inner_bounds, Self::RADIUS, egui::Color32::GRAY);
        response
    }
}
