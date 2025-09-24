use eframe::{egui, App, Frame};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Step 01 - Hello Window",
        options,
         Box::new(|_cc| Ok(Box::new(HelloApp))),
    )
}

struct HelloApp;

impl App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }
}
