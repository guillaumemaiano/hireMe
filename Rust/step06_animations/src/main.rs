use eframe::{
    App, Frame,
    egui::{self, ColorImage},
};
use egui_extras::{Column, TableBuilder};

// ============ Local modules ============
mod i18n_wrapper;
use i18n_wrapper::I18n;
mod hire_me_model;
use hire_me_model::HireMeModel;
mod fonts;
use fonts::setup_fonts;
mod spy;
mod spy_data_view;

use crate::spy::spy_builder::{EguiTextureProvider, SpyBuilder};
use crate::spy::{SpyInfo, SpyScript};
use crate::spy_data_view::SpyDataView;

// ============ App metadata ============
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// ==========================================================
//  Entry point — initialize eframe and all subsystems
// ==========================================================
fn main() -> eframe::Result<()> {
    // --- load optional window icon ---
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

    // --- configure viewport ---
    let mut viewport = egui::ViewportBuilder::default();
    if let Some(icon) = icon {
        viewport = viewport.with_icon(std::sync::Arc::new(icon));
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    // ======================================================
    //  Run the application
    // ======================================================
    eframe::run_native(
        &format!("{} v{}", APP_NAME, APP_VERSION),
        options,
        Box::new(|cc| {
            // --- fonts ---
            let fonts = setup_fonts();
            cc.egui_ctx.set_fonts(fonts);

            // --- load header photo texture ---
            let photo = image::open("assets/GEM_EXED_MIT.png")
                .expect("photo missing")
                .to_rgba8();
            let size = [photo.width() as usize, photo.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &photo);
            let texture = cc
                .egui_ctx
                .load_texture("my_photo", color_image, Default::default());

            // --- spy scripts ---
            let script_intro = SpyScript::new(vec![SpyInfo::TextBlock {
                lines: vec!["Hello agent.".into(), "Your mission begins now.".into()],
                chars_per_sec: 2.0,
                duration: 20.0,
            }]);
            let script_lorem = SpyScript::from_assets("assets/spy", "lorem").unwrap_or_else(|e| {
                eprintln!("{e}");
                SpyScript::new(vec![])
            });

            // --- build renderables once ---
            let mut provider = EguiTextureProvider::new(&cc.egui_ctx);
            let view_intro = SpyDataView::new(SpyBuilder::build(&script_intro, &mut provider));
            let view_lorem = SpyDataView::new(SpyBuilder::build(&script_lorem, &mut provider));

            // --- persistent spy manager ---
            let spy_manager = SpyManager::new(vec![view_intro, view_lorem]);

            Ok(Box::new(HelloApp {
                i18n: I18n::new("en"),
                model: HireMeModel::new(),
                texture,
                spy_manager,
            }))
        }),
    )
}

// ==========================================================
//  SpyManager — handles multiple animated sequences
// ==========================================================
struct SpyManager {
    views: Vec<SpyDataView>,
}

impl SpyManager {
    fn new(views: Vec<SpyDataView>) -> Self {
        Self { views }
    }

    fn update_and_draw(&mut self, dt: f32, ui: &mut egui::Ui) {
        for view in &mut self.views {
            view.update(dt);
            view.draw(ui);
            ui.add_space(8.0);
        }
    }
}

// ==========================================================
//  HelloApp — main application
// ==========================================================
struct HelloApp {
    i18n: I18n,
    model: HireMeModel,
    texture: egui::TextureHandle,
    spy_manager: SpyManager,
}

// ----------------------------------------------------------
//  UI update loop — called each frame by eframe
// ----------------------------------------------------------
impl App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // --- Audience & Language selection ---
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

            // --- Intro text ---
            let intro = egui::RichText::new(self.i18n.t("intro"))
                .font(egui::FontId::new(32.0, egui::FontFamily::Proportional));

            // =============================
            //  Résumé layout (original UI)
            // =============================
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    let avail = ui.available_width();
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
                                        .column(Column::remainder().resizable(true).clip(false))
                                        .column(Column::remainder().resizable(true).clip(false))
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
                                                args.set(
                                                    "langName",
                                                    HireMeModel::lang_name(&lp.lang),
                                                );
                                                args.set(
                                                    "level",
                                                    match lp.fluency {
                                                        hire_me_model::Fluency::Fluent => "fluent",
                                                        hire_me_model::Fluency::Learning => {
                                                            "learning"
                                                        }
                                                    },
                                                );
                                                body.row(20.0, |mut row| {
                                                    row.col(|ui| {
                                                        ui.label(self.i18n.t_with_args(
                                                            "language-profile",
                                                            &args,
                                                        ));
                                                    });
                                                    row.col(|ui| {
                                                        ui.label(
                                                            self.i18n.t_with_args("fluency", &args),
                                                        );
                                                    });
                                                });
                                            }
                                        });
                                }
                                hire_me_model::Audience::Business
                                | hire_me_model::Audience::Other => {
                                    let args = self.model.to_args();
                                    ui.label(self.i18n.t_with_args("language-profile", &args));
                                }
                            }
                        });
                    });
                });

                ui.add_space(12.0);

                // --- Scrollable “about me” block ---
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    let args = self.model.to_args();
                                    ui.label(self.i18n.t_with_args("audience", &args));
                                    ui.label(self.i18n.t_with_args("fluency", &args));
                                    ui.label(self.i18n.t_with_args("language-profile", &args));
                                });
                                ui.add_space(12.0);
                                ui.hyperlink_to(
                                    egui::RichText::new(self.i18n.t("website"))
                                        .underline()
                                        .size(20.0),
                                    "https://guillaume.maiano.fr",
                                );
                            });
                        });
                });

                // ===============================
                //  Spy sequences (animated block)
                // ===============================
                ui.add_space(20.0);
                ui.separator();
                ui.heading("Spy Sequences");

                let dt = ctx.input(|i| i.stable_dt);
                self.spy_manager.update_and_draw(dt, ui);
            });
        });
    }
}
