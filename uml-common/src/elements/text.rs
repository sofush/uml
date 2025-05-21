use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextProperties {
    size: f32,
    font: String,
}

impl TextProperties {
    pub fn new(size: f32, font: impl Into<String>) -> Self {
        Self {
            size,
            font: font.into(),
        }
    }

    pub fn get_font_string(&self) -> String {
        format!("{}px {}", self.size, self.font)
    }
}

impl Default for TextProperties {
    fn default() -> Self {
        Self {
            size: 20.0,
            font: String::from("Arial,sans-serif"),
        }
    }
}
