pub struct DebugPanel {
    translation: egui::Vec2,
    scale: f32,
}

impl DebugPanel {
    pub fn new() -> Self {
        Self {
            translation: egui::Vec2::ZERO,
            scale: 1.0,
        }
    }

    pub fn update_transform(&mut self, response: &egui::Response, zoom: f32) {
        self.translation += response.drag_delta();

        // If the zoom is 1.0, we don't need to do everything else after.
        if zoom == 1.0 || !response.hovered() {
            return;
        }
        let prev_scale = self.scale;
        if zoom < 1.0 {
            if self.scale > 0.2 {
                self.scale -= 0.1;
            }
        } else {
            self.scale += 0.1;
        }
        if let Some(absolute_cursor) = response.hover_pos() {
            let factor = self.scale / prev_scale;
            let cursor = absolute_cursor - response.rect.min;
            self.translation.x = factor * self.translation.x + (1.0 - factor) * cursor.x;
            self.translation.y = factor * self.translation.y + (1.0 - factor) * cursor.y;
        }
    }

    fn position(&self, x: f32, y: f32) -> egui::Pos2 {
        (egui::Pos2::new(x, y) * self.scale) + self.translation
    }

    fn size(&self, size: f32) -> f32 {
        size * self.scale
    }
}

impl egui::Widget for &mut DebugPanel {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) = ui.allocate_painter(ui.available_size(),
            egui::Sense::click_and_drag());
        self.update_transform(&response, ui.input(|i| i.zoom_delta()));


        painter.circle_filled(self.position(100.0, 100.0), self.size(50.0), egui::Color32::RED);

        response
    }
}
