use super::{SpyInfo, SpyLine, SpyPic, SpyRenderable, SpyScript};
use crate::egui::{ColorImage, Context, TextureId, TextureOptions};

/// Converts a `SpyScript` into concrete renderable objects.
///

pub struct SpyBuilder;

pub trait TextureProvider {
    fn load(&mut self, path: &str) -> TextureId;
}

pub struct EguiTextureProvider<'a> {
    ctx: &'a Context,
}

impl<'a> EguiTextureProvider<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        Self { ctx }
    }
}

impl<'a> TextureProvider for EguiTextureProvider<'a> {
    fn load(&mut self, path: &str) -> TextureId {
        let image_bytes =
            std::fs::read(path).unwrap_or_else(|_| panic!("Cannot read image file '{}'", path));
        let image = image::load_from_memory(&image_bytes)
            .expect("Unsupported image format")
            .to_rgba8();
        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.into_raw();

        self.ctx
            .load_texture(
                path.to_owned(),
                ColorImage::from_rgba_unmultiplied(size, &pixels),
                TextureOptions::LINEAR,
            )
            .id()
    }
}

impl SpyBuilder {
    pub fn build(
        script: &SpyScript,
        provider: &mut dyn TextureProvider,
    ) -> Vec<Box<dyn SpyRenderable>> {
        let mut items: Vec<Box<dyn SpyRenderable>> = Vec::new();

        for info in &script.items {
            match info {
                SpyInfo::TextBlock {
                    lines,
                    chars_per_sec,
                    ..
                } => {
                    for line in lines {
                        items.push(Box::new(SpyLine::new(line.clone(), *chars_per_sec)));
                    }
                }
                SpyInfo::Picture {
                    path,
                    segments,
                    duration,
                    ..
                } => {
                    let tex_id = provider.load(path);
                    items.push(Box::new(SpyPic::new(tex_id, *segments, *duration)));
                }
            }
        }

        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockProvider {
        loaded_paths: RefCell<Vec<String>>,
    }

    impl MockProvider {
        fn new() -> Self {
            Self { loaded_paths: RefCell::new(Vec::new()) }
        }
    }

    impl TextureProvider for MockProvider {
        fn load(&mut self, path: &str) -> TextureId {
            self.loaded_paths.borrow_mut().push(path.to_string());
            TextureId::User(42)
        }
    }

    #[test]
    fn builds_text_and_picture_items() {
        let script = SpyScript::new(vec![
            SpyInfo::TextBlock {
                lines: vec!["Hello".into(), "World".into()],
                chars_per_sec: 12.0,
                duration: 5.0,
            },
            SpyInfo::Picture {
                path: "assets/test.png".into(),
                segments: 9,
                duration: 7.5,
                max_width: 400.0,
            },
        ]);

        let mut provider = MockProvider::new();
        let items = SpyBuilder::build(&script, &mut provider);

        // 2 lines of text + 1 picture
        assert_eq!(items.len(), 3);

        // Picture loading called once, correct path recorded
        let paths = provider.loaded_paths.borrow();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], "assets/test.png");
    }
}
