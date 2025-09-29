use egui;
use crate::spy::spy_view::SpyRenderable;

/// A line of text that reveals itself character by character.
pub struct SpyLine {
    text: String,
    revealed_chars: usize,
    chars_per_sec: f32,
}

impl SpyLine {
    pub fn new(text: String, chars_per_sec: f32) -> Self {
        Self {
            text,
            revealed_chars: 0,
            chars_per_sec,
        }
    }
}

impl SpyRenderable for SpyLine {
    fn update(&mut self, dt: f32) -> bool {
        let inc = (dt * self.chars_per_sec).floor() as usize;
        self.revealed_chars = (self.revealed_chars + inc).min(self.text.len());
        self.revealed_chars == self.text.len()
    }

    fn draw(&self, ui: &mut egui::Ui) {
        let shown = &self.text[..self.revealed_chars];
        ui.label(shown);
    }
}
