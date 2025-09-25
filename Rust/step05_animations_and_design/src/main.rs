mod i18n_wrapper;
use i18n_wrapper::I18n;
mod hire_me_model;
use hire_me_model::HireMeModel;
mod fonts;
use fonts::setup_fonts;

use eframe::{
    App, Frame,
    egui::{self, ColorImage, Context, FontData, FontDefinitions, FontFamily, TextureHandle, Vec2},
};
use image::GenericImageView;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    // image
    let photo = image::open("assets/GEM_EXED_MIT.png")
        .expect("photo missing")
        .to_rgba8();
    let size = [photo.width() as usize, photo.height() as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &photo);
    // frame box
    eframe::run_native(
        &format!("{} v{}", APP_NAME, APP_VERSION),
        options,
        Box::new(|cc| {
            // Create texture handle inside the context
            let texture = cc
                .egui_ctx
                .load_texture("my_photo", color_image, Default::default());
            // fonts come from a font struct file
            let fonts = setup_fonts();
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(HelloApp {
                i18n: I18n::new("en"),
                model: HireMeModel::new(),
                texture: texture,
            }))
        }),
    )
}

struct HelloApp {
    i18n: I18n,
    model: HireMeModel,
    texture: egui::TextureHandle,
}

impl App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let intro = egui::RichText::new(self.i18n.t("intro"))
                .font(egui::FontId::new(32.0, egui::FontFamily::Proportional));

            ui.label(intro);
            let desired_width = 400.0;
            let aspect = self.texture.size()[1] as f32 / self.texture.size()[0] as f32;
            let desired_size = egui::vec2(desired_width, desired_width * aspect);
            ui.add(egui::Image::new(&self.texture).fit_to_exact_size(desired_size));

            let special = egui::RichText::new(self.i18n.t("hireme"))
                .font(egui::FontId::new(32.0, egui::FontFamily::Proportional))
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

            let args = self.model.to_args();
            ui.label(self.i18n.t_with_args("audience", &args));
            ui.label(self.i18n.t_with_args("fluency", &args));
            ui.label(self.i18n.t_with_args("language-profile", &args));

            ui.hyperlink_to(
                egui::RichText::new(self.i18n.t("website"))
                    .underline()
                    .size(20.0),
                "https://guillaume.maiano.fr",
            );
        });
    }
}
