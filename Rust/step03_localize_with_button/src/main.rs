mod i18n_wrapper;
use i18n_wrapper::I18n;
mod hire_me_model;
use hire_me_model::HireMeModel;

use eframe::egui::{FontData, FontDefinitions, FontFamily};
use eframe::{egui, App, Frame};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    // frame box
    eframe::run_native(
        &format!("{} v{}", APP_NAME, APP_VERSION),
        options,
        Box::new(|cc| {
            // fonts -- I want CrimsonText
            let mut fonts = FontDefinitions::default();

            // Add font file
            fonts.font_data.insert(
                "my_font".to_owned(),
                FontData::from_owned(include_bytes!("../assets/CrimsonText-Regular.ttf").to_vec())
                    .into(),
            );

            // Create a new family alias "MyFont"
            fonts.families.insert(
                FontFamily::Name("MyFont".into()),
                vec!["my_font".to_owned()],
            );

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(HelloApp {
                i18n: I18n::new("en"),
            }))
        }),
    )
}

struct HelloApp {
    i18n: I18n,
}

impl App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(self.i18n.t("intro"));
            let special = egui::RichText::new(self.i18n.t("hireme"))
                .font(egui::FontId::new(
                    32.0,
                    egui::FontFamily::Name("MyFont".into()),
                ))
                .color(egui::Color32::from_rgb(220, 40, 40));
            ui.label(special);
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            ui.hyperlink_to(
                egui::RichText::new(self.i18n.t("website"))
                    .underline()
                    .size(20.0),
                "https://guillaume.maiano.fr",
            );
        });
    }
}
