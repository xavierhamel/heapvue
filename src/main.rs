mod components;
mod debug_panel;
mod alloc;

use std::process;
use eframe::egui;

const SIDE_PANEL_WIDTH: f32 = 300.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Fast Image Format Debugger",
        options,
        Box::new(|creation_context| Box::new(App::new(creation_context))),
    )
}

struct App {
    memory: components::Memory,
}

impl App {
    pub fn new(creation_context: &eframe::CreationContext) -> Self {
        let mut command = process::Command::new("python");
        command.arg("./test.py");
        let chunks = alloc::Chunks::new(command, creation_context.egui_ctx.clone());
        Self {
            memory: components::Memory::new(chunks),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("sidepanel")
            .min_width(SIDE_PANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(16.0);
                ui.add(components::SectionTitle(String::from("Controls")));
                let play_pause_button_label = match self.memory.do_advance() {
                    true => "Pause",
                    false => "Play",
                };
                if ui.button(play_pause_button_label).clicked() {
                    self.memory.set_do_advance(!self.memory.do_advance());
                }
                if let Some(chunk) = self.memory.selected_chunk() {
                    ui.add_space(16.0);
                    let ptr = format!("{:#01x}", chunk.ptr);
                    let size = format!("{} bytes", chunk.size);
                    ui.add(components::SectionTitle(String::from("Chunk")));
                    ui.add(components::Field::new("Ptr", &ptr));
                    ui.add(components::Field::new("Size", &size));
                    ui.add(components::Field::new("Identifiers", &chunk.identifier));
                    ui.add(components::Field::new("State", &chunk.state.to_string()));
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(&mut self.memory);
        });
    }
}
