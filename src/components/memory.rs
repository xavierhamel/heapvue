use crate::alloc;

pub const BYTES_PER_LINE: u64 = 1024;
const LINE_HEIGHT_PX: u64 = 32;
const BYTE_WIDTH_PX: u64 = 3;
pub const MAX_LINE_COUNT: u64 = 262144 / BYTES_PER_LINE;

pub const COLOR_USED: egui::Color32 = egui::Color32::from_rgb(41, 128, 185);
pub const COLOR_ALREADY_USED: egui::Color32 = egui::Color32::from_rgb(231, 76, 60);
pub const COLOR_CORRUPTED: egui::Color32 = egui::Color32::from_rgb(142, 68, 173);
const COLOR_SELECTED: egui::Color32 = egui::Color32::LIGHT_GRAY;
const COLOR_HOVERD: egui::Color32 = egui::Color32::from_rgb(142, 68, 173);

pub struct Memory {
    chunks: alloc::Chunks,
    translation: egui::Vec2,
    scale: f32,
    selected_ptr: Option<u64>,
}

impl Memory {
    pub fn new(chunks: alloc::Chunks) -> Self {
        Self {
            chunks,
            translation: egui::Vec2::ZERO,
            scale: 0.5,
            selected_ptr: None,
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
            if self.scale > 0.06 {
                self.scale -= 0.03;
            }
        } else {
            self.scale += 0.03;
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

    fn rect_from_x_y_size(&self, x: u64, y: u64, size: u64) -> egui::Rect {
        let position = self.position((x * BYTE_WIDTH_PX) as f32, (y * LINE_HEIGHT_PX) as f32);
        let size = egui::Vec2::new(self.size((size * BYTE_WIDTH_PX) as f32),
            self.size(LINE_HEIGHT_PX as f32));
        egui::Rect::from_min_size(position, size)
    }

    fn rect_from_ptr_and_size(&self, ptr: u64, size: u64) -> egui::Rect {
        let position = self.position(((ptr % BYTES_PER_LINE) * BYTE_WIDTH_PX) as f32,
            ((ptr / BYTES_PER_LINE) * LINE_HEIGHT_PX) as f32);
        let size = egui::Vec2::new(self.size((size * BYTE_WIDTH_PX) as f32),
            self.size(LINE_HEIGHT_PX as f32));
        egui::Rect::from_min_size(position, size)
    }

    fn chunk_to_rects(&self, chunk: &alloc::Chunk) -> (egui::Color32, Vec<egui::Rect>) {
        let start_x = chunk.ptr % BYTES_PER_LINE;
        let mut rects = Vec::new();
        let color = chunk.state.to_color();
        if start_x + chunk.size < BYTES_PER_LINE {
            return (color, vec![self.rect_from_ptr_and_size(chunk.ptr, chunk.size)]);
        } else {
            rects.push(self.rect_from_ptr_and_size(chunk.ptr, BYTES_PER_LINE - start_x));
        }
        if chunk.size < BYTES_PER_LINE - start_x {
            return (color, rects);
        }
        let mut remaining = chunk.size - (BYTES_PER_LINE - start_x);
        let mut y = chunk.ptr / BYTES_PER_LINE + 1;
        loop {
            if remaining < BYTES_PER_LINE {
                rects.push(self.rect_from_x_y_size(0, y, remaining));
                break;
            }
            rects.push(self.rect_from_x_y_size(0, y, BYTES_PER_LINE));
            y += 1;
            remaining -= BYTES_PER_LINE;
        }
        (color, rects)
    }

    fn is_cell_hovered(&self, maybe_cursor: Option<egui::Pos2>, rects: &[egui::Rect]) -> bool {
        if let Some(cursor) = maybe_cursor {
            return rects.iter().any(|rect| rect.contains(cursor));
        }
        return false;
    }

    pub fn selected_chunk(&self) -> Option<alloc::Chunk> {
        if let Some(ptr) = self.selected_ptr {
            return Some((*self.chunks.get(ptr)?).clone());
        }
        return None;
    }

    pub fn set_do_advance(&mut self, do_advance: bool) {
        self.chunks.do_advance = do_advance;
    }

    pub fn do_advance(&self) -> bool {
        self.chunks.do_advance
    }
}

impl egui::Widget for &mut Memory {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        self.chunks.update();
        let (response, painter) = ui.allocate_painter(ui.available_size(),
            egui::Sense::click_and_drag());
        self.update_transform(&response, ui.input(|i| i.zoom_delta()));

        let clicked = response.clicked();
        let maybe_hover_pos = ui.ctx().input(|i| i.pointer.hover_pos());

        let mut did_select_cell = false;
        for (&chunk_ptr, chunk) in self.chunks.iter() {
            let (mut color, rects) = self.chunk_to_rects(chunk);
            if self.is_cell_hovered(maybe_hover_pos, &rects) {
                color = COLOR_HOVERD;
                if clicked {
                    did_select_cell = true;
                    self.selected_ptr = Some(chunk_ptr);
                }
            }
            let is_selected = self.selected_ptr.map_or(false, |ptr| ptr == chunk_ptr);
            for rect in rects {
                painter.rect_filled(rect, 3.0, color);
                if is_selected {
                    painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.0, COLOR_SELECTED));
                }
            }
        }
        if !did_select_cell && clicked {
            self.selected_ptr = None;
        }

        let border_position = self.position(0.0, 0.0);
        let border_size = egui::Vec2::new(self.size((BYTES_PER_LINE * BYTE_WIDTH_PX) as f32),
            self.size((LINE_HEIGHT_PX * MAX_LINE_COUNT) as f32));
        let border_rect = egui::Rect::from_min_size(border_position, border_size);
        painter.rect_stroke(border_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));

        response
    }
}
