use std::fs;
use std::path::Path;

use serde::Deserialize;

/// Provide a default duration (5s) for picture reveal animations.
fn default_duration() -> f32 {
    5.0
}

/// Provide a default maximum width (400 points/pixels).
fn default_max_width() -> f32 {
    400.0
}

/// A single information block in the spy sequence.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")] // add a "type" field in JSON to pick variant
pub enum SpyInfo {
    /// A block of text lines, revealed with typing.
    TextBlock {
        lines: Vec<String>,
        chars_per_sec: f32,
        /// Optional duration: how long to display before auto-advance.
        /// `None` = stays until user action.
        #[serde(default)]
        duration: Option<f32>,
    },

    /// A picture, revealed progressively.
    Picture {
        path: String,          // later resolved into a texture
        segments: usize,
        /// Duration always has a value; defaults to 5s if not provided.
        #[serde(default = "default_duration")]
        duration: f32,
        /// Max width always has a value; defaults to 400 if not provided.
        #[serde(default = "default_max_width")]
        max_width: f32,
    },
}

/// A sequence of spy information items.
#[derive(Debug, Clone, Deserialize)]
pub struct SpyScript {
    pub items: Vec<SpyInfo>,
}

impl SpyScript {
    /// Construct directly in code.
    pub fn new(items: Vec<SpyInfo>) -> Self {
        Self { items }
    }

    /// Validate script constraints (e.g., picture segments).
    pub fn validate(&self) -> Result<(), String> {
        for (i, item) in self.items.iter().enumerate() {
            match item {
                SpyInfo::Picture { segments, .. } => {
                    if *segments == 0 || *segments > 10 {
                        return Err(format!(
                            "Item {}: Picture segments must be between 1 and 10, got {}",
                            i, segments
                        ));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Load a SpyScript from a JSON file in the `assets/` folder.
    pub fn from_assets<P: AsRef<Path>>(assets_dir: P, name: &str) -> Result<Self, String> {
        let path = assets_dir.as_ref().join(format!("{name}.json"));
        let data = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
        let script: SpyScript = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))?;

        script.validate()?;
        Ok(script)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_textblock_from_json() {
        let json = r#"
        {
            "items": [
                {
                    "type": "TextBlock",
                    "lines": ["Hello", "World"],
                    "chars_per_sec": 15.0,
                    "duration": null
                }
            ]
        }"#;

        let script: SpyScript = serde_json::from_str(json).unwrap();
        assert_eq!(script.items.len(), 1);

        match &script.items[0] {
            SpyInfo::TextBlock { lines, chars_per_sec, duration } => {
                assert_eq!(lines, &vec!["Hello".into(), "World".into()]);
                assert_eq!(*chars_per_sec, 15.0);
                assert!(duration.is_none());
            }
            _ => panic!("Expected TextBlock"),
        }
    }

    #[test]
    fn parse_picture_with_all_fields() {
        let json = r#"
        {
            "items": [
                {
                    "type": "Picture",
                    "path": "assets/test.png",
                    "segments": 9,
                    "duration": 7.5,
                    "max_width": 300.0
                }
            ]
        }"#;

        let script: SpyScript = serde_json::from_str(json).unwrap();
        match &script.items[0] {
            SpyInfo::Picture { path, segments, duration, max_width } => {
                assert_eq!(path, "assets/test.png");
                assert_eq!(*segments, 9);
                assert_eq!(*duration, 7.5);
                assert_eq!(*max_width, 300.0);
            }
            _ => panic!("Expected Picture"),
        }
    }

    #[test]
    fn parse_picture_without_duration_or_max_width_uses_defaults() {
        let json = r#"
        {
            "items": [
                {
                    "type": "Picture",
                    "path": "assets/test.png",
                    "segments": 9
                }
            ]
        }"#;

        let script: SpyScript = serde_json::from_str(json).unwrap();
        match &script.items[0] {
            SpyInfo::Picture { duration, max_width, .. } => {
                assert_eq!(*duration, 5.0);   // default applied
                assert_eq!(*max_width, 400.0); // default applied
            }
            _ => panic!("Expected Picture"),
        }
    }

    #[test]
    fn picture_with_invalid_segments_fails() {
        let json = r#"
        {
            "items": [
                {
                    "type": "Picture",
                    "path": "assets/test.png",
                    "segments": 0
                }
            ]
        }"#;

        let script: SpyScript = serde_json::from_str(json).unwrap();
        assert!(script.validate().is_err());
    }
}
