use eframe::egui;
use crate::spy::spy_view::SpyRenderable;

pub struct SpyPic {
    texture_id: egui::TextureId, // handle to the loaded image
    total_segments: usize,
    revealed_segments: usize,
    duration: f32,    // seconds to reveal fully
    progress: f32,    // 0.0 → duration
}

impl SpyPic {
    pub fn new(texture_id: egui::TextureId, total_segments: usize, duration: f32) -> Self {
        Self {
            texture_id,
            total_segments,
            revealed_segments: 0,
            duration,
            progress: 0.0,
        }
    }
}
// ===========================================
impl SpyRenderable for SpyPic {
    fn start(&mut self) {
        self.progress = 0.0;
        self.revealed_segments = 0;
    }

    fn update(&mut self, dt: f32) -> bool {
        self.progress += dt;
        let ratio = (self.progress / self.duration).min(1.0);
        self.revealed_segments = (ratio * self.total_segments as f32).ceil() as usize;
        ratio >= 1.0
    }

    fn draw(&self, ui: &mut egui::Ui) {
        // Full available space for now (you can size this)
        let available = ui.available_size();
        let img_size = available.min_elem(); // square-ish

        let segments_per_row = (self.total_segments as f32).sqrt().ceil() as usize;
        let seg_w = img_size / segments_per_row as f32;

        for row in 0..segments_per_row {
            for col in 0..segments_per_row {
                let idx = row * segments_per_row + col;
                if idx < self.revealed_segments {
                    let rect = egui::Rect::from_min_size(
                        ui.min_rect().min + egui::vec2(col as f32 * seg_w, row as f32 * seg_w),
                        egui::vec2(seg_w, seg_w),
                    );
                    ui.painter().image(
                        self.texture_id,
                        rect,
                        egui::Rect::from_min_size(
                            egui::pos2(col as f32 / segments_per_row as f32,
                                       row as f32 / segments_per_row as f32),
                            egui::vec2(1.0 / segments_per_row as f32,
                                       1.0 / segments_per_row as f32),
                        ),
                        egui::Color32::WHITE,
                    );
                }
            }
        }
    }
}
