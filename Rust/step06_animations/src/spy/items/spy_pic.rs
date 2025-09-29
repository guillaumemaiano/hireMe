use egui;
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
