mod i18n_wrapper;
use i18n_wrapper::I18n;

use eframe::{egui, App, Frame};
use eframe::egui::{FontData, FontDefinitions, FontFamily};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    // frame box
    eframe::run_native(
        "Step 01 - Hello Window",
        options,
         Box::new(|cc| {
             // fonts -- I want CrimsonText
      let mut fonts = FontDefinitions::default();

            // Add font file
            fonts.font_data.insert(
                "my_font".to_owned(),
                FontData::from_owned(include_bytes!("../assets/CrimsonText-Regular.ttf").to_vec()).into(),
            );

            // Create a new family alias "MyFont"
            fonts.families.insert(
                FontFamily::Name("MyFont".into()),
                vec!["my_font".to_owned()],
            );

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(HelloApp))}),
        
    )
}

struct HelloApp;

impl App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
            let special = egui::RichText::new("I'm available for hire!")
                .font(egui::FontId::new(32.0, egui::FontFamily::Name("MyFont".into())));
            ui.colored_label(egui::Color32::RED, special);
        });
    }
}
