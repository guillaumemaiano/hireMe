mod spy;

use crate::spy::{SpyRenderable, SpyLine};
use crate::spy::{SpyLine, SpyPic, SpyRenderable};

pub struct SpyDataView {
}

impl SpyDataView {
    pub fn new(script: SpyScript, texture_loader: &mut TextureLoader) -> Self {
        let mut items: Vec<Box<dyn SpyRenderable>> = Vec::new();

        for info in script.items {
            match info {
                SpyInfo::TextBlock { lines, chars_per_sec, .. } => {
                    for line in lines {
                        items.push(Box::new(SpyLine::new(line, chars_per_sec)));
                    }
                }
                SpyInfo::Picture { path, segments, duration } => {
                    let tex_id = texture_loader.load(&path);
                    items.push(Box::new(SpyPic::new(tex_id, segments, duration.unwrap_or(5.0))));
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.view.update(dt)
    }

    pub fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        self.view.draw(ui);
    }
}
