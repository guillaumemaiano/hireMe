//! SpyView: displays text, pictures and videos in a
//! "spy movie" style reveal animation.
//!
//! Example usage:
//! ```no_run
//! let mut view = SpyView::new(items);
//! ```

// Note: I initially wanted to be library independant, but it is not sensible for a demo project that already chose egui.
use egui;

/// A trait for any item that can be revealed inside a [`SpyView`].
///
/// Each `SpyRenderable` represents one step in a spy-style sequence:
/// - a line of text revealed character by character,
/// - a picture that fades in,
/// - a video that plays,
/// - or even a nested [`SpyView`].
///
/// The trait provides a simple life-cycle:
///
/// 1. [`start`] is called once when the item begins.
/// 2. [`update`] is called every frame with a `dt` in seconds,
///    and should advance the animation.  
///    It must return `true` when the item is fully revealed/done.
/// 3. [`draw`] is called every frame to render the current state.
///
/// # Examples
/// A minimal “typing” line:
/// ```no_run
/// struct MyLine { text: String, shown: usize }
///
/// impl SpyRenderable for MyLine {
///     fn update(&mut self, _dt: f32) -> bool {
///         self.shown = self.text.len();
///         true
///     }
///
///     fn draw(&self, ui: &mut egui::Ui) {
///         ui.label(&self.text[..self.shown]);
///     }
/// }
/// ```
/// # Notes for implementors -- eg, probably just me in a few months ;)
/// - Keep `update` idempotent: once it returns `true`, further calls should
///   continue to return `true`.
/// - If you need to reset an item for reuse, provide a custom `reset()` method
///   on your type; don’t overload `start`.

pub trait SpyRenderable {
    /// Called once when the item begins.
    fn start(&mut self) {}

    /// Advance animation by `dt` seconds.
    ///
    /// Returns `true` once the item has fully finished revealing.
    fn update(&mut self, dt: f32) -> bool;

    /// Draw the current state into the provided egui UI.
    fn draw(&self, ui: &mut egui::Ui);
}