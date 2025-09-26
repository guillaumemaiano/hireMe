mod i18n_wrapper;
use i18n_wrapper::I18n;
mod hire_me_model;
use hire_me_model::HireMeModel;
mod fonts;
use fonts::setup_fonts;

use eframe::{
    App, Frame,
    egui::{self},
};
use egui_extras::{Column, TableBuilder};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> eframe::Result<()> {
    let icon = match image::open("assets/topGem.png") {
        Ok(img) => {
            let img = img.to_rgba8();
            let (w, h) = img.dimensions();
            Some(egui::IconData {
                rgba: img.into_raw(),
                width: w,
                height: h,
            })
        }
        Err(_) => None,
    };
    let mut viewport = egui::ViewportBuilder::default();
    if let Some(icon) = icon {
        viewport = viewport.with_icon(std::sync::Arc::new(icon));
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
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

            egui::Frame::group(ui.style()).show(ui, |ui| {
                let avail = ui.available_width();
                // 60% of screen, max 600 px -- Labels inside the "group"
                // Should look OK on reasonable-size screens?
                let target = (avail * 0.6).clamp(300.0, 600.0);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    let w = 250.0;
                    let aspect = self.texture.size()[1] as f32 / self.texture.size()[0] as f32;
                    let size = egui::vec2(w, w * aspect);
                    ui.add(egui::Image::new(&self.texture).fit_to_exact_size(size));

                    ui.vertical(|ui| {
                        ui.set_max_width(target);
                        ui.label(intro);

                        let special = egui::RichText::new(self.i18n.t("hireme"))
                            .font(egui::FontId::new(32.0, egui::FontFamily::Proportional))
                            .color(egui::Color32::from_rgb(220, 40, 40));
                        ui.label(special);

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(4.0);

                        match self.model.audience {
                            hire_me_model::Audience::Academic => {
                                ui.separator();
                                ui.heading("Languages");

                                TableBuilder::new(ui)
                                    .striped(true)
                                    .column(Column::auto())
                                    .column(Column::remainder())
                                    .header(20.0, |mut header| {
                                        header.col(|ui| {
                                            ui.label(self.i18n.t("table-language"));
                                        });
                                        header.col(|ui| {
                                            ui.label(self.i18n.t("table-level"));
                                        });
                                    })
                                    .body(|mut body| {
                                        for lp in self.model.languages_with_fluency() {
                                            let mut args = self.model.to_args();
                                            args.set("lang", HireMeModel::lang_code(&lp.lang));
                                            args.set("langName", HireMeModel::lang_name(&lp.lang)); // fallback
                                            args.set(
                                                "level",
                                                match lp.fluency {
                                                    hire_me_model::Fluency::Fluent => "fluent",
                                                    hire_me_model::Fluency::Learning => "learning",
                                                },
                                            );
                                            body.row(20.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.label(
                                                    self.i18n
                                                        .t_with_args("language-profile", &args));
                                                });
                                                row.col(|ui| {
                                                      ui.label(self.i18n.t_with_args("fluency", &args));
                                                });
                                            });
                                        }
                                    });
                            }
                            hire_me_model::Audience::Business => {
                                let args = self.model.to_args();
                                ui.label(self.i18n.t_with_args("language-profile", &args));
                            }
                            hire_me_model::Audience::Other => {
                                let args = self.model.to_args();
                                ui.label(self.i18n.t_with_args("language-profile", &args));
                            }
                        }
                    });
                });
            });

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
