use crate::spy::SpyRenderable;

pub struct SpyDataView {
    items: Vec<Box<dyn SpyRenderable>>,
}

impl SpyDataView {
    pub fn new(items: Vec<Box<dyn SpyRenderable>>) -> Self {
        Self { items }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        let mut all_done = true;
        for item in &mut self.items {
            if !item.update(dt) {
                all_done = false;
            }
        }
        all_done
    }

    pub fn draw(&self, ui: &mut eframe::egui::Ui) {
        for item in &self.items {
            item.draw(ui);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui;
    use std::cell::RefCell;

    struct DummyRenderable {
        updates: RefCell<u32>,
        done_after: u32,
    }

    impl DummyRenderable {
        fn new(done_after: u32) -> Self {
            Self {
                updates: RefCell::new(0),
                done_after,
            }
        }
    }

    impl SpyRenderable for DummyRenderable {
        fn update(&mut self, _dt: f32) -> bool {
            let mut n = self.updates.borrow_mut();
            *n += 1;
            *n >= self.done_after
        }
        fn draw(&self, _ui: &mut egui::Ui) {}
    }

    #[test]
    fn update_returns_false_until_all_done() {
        let items: Vec<Box<dyn SpyRenderable>> = vec![
            Box::new(DummyRenderable::new(2)),
            Box::new(DummyRenderable::new(3)),
        ];
        let mut view = SpyDataView::new(items);
        // first call: not all done
        assert!(!view.update(1.0));
        // second call: still not all done
        assert!(!view.update(1.0));
        // third call: all done
        assert!(view.update(1.0));
    }
}
