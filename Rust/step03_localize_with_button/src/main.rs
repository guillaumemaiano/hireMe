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
                model: HireMeModel::new(),
            }))
        }),
    )
}

struct HelloApp {
    i18n: I18n,
    model: HireMeModel,
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
/* 
            ui.add_space(12.0);
            ui.label(format!(
                "Audience: {:?}, Language: {}, Fluency: {:?}",
                self.model.audience,
                self.model.current_lang_name(),
                self.model.fluency
            )); */

            ui.add_space(12.0);

            ui.horizontal(|ui| {
            egui::ComboBox::from_label("Audience")
                .selected_text(self.model.current_audience_name())
                .show_ui(ui, |ui| {
                    for name in HireMeModel::all_audiences() {
                        if ui
                            .selectable_label(self.model.current_audience_name() == name, name)
                            .clicked()
                        {
                            self.model.set_audience_by_name(name);
                            self.model.set_lang_default();
                            self.i18n.set_lang("en");
                        }
                    }
                });
            egui::ComboBox::from_label("Language")
                .selected_text(self.model.current_lang_name())
                .show_ui(ui, |ui| {
                    for lang in self.model.available_languages() {
                        let name = HireMeModel::lang_name(&lang);
                        if ui
                            .selectable_label(self.model.current_lang == lang, name)
                            .clicked()
                        {
                            self.model.set_language(lang.clone());
                            self.i18n.set_lang(HireMeModel::lang_code(&lang));
                        }
                    }
                });
            });

            ui.hyperlink_to(
                egui::RichText::new(self.i18n.t("website"))
                    .underline()
                    .size(20.0),
                "https://guillaume.maiano.fr",
            );
        });
    }
}
