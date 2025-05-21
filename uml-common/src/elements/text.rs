use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextProperties {
    size: f32,
    font: String,
    weight: Option<u32>,
}

impl TextProperties {
    pub fn new(size: f32, font: impl Into<String>) -> Self {
        Self {
            size,
            font: font.into(),
            weight: None,
        }
    }

    pub fn get_font_string(&self) -> String {
        let weight = self.weight.unwrap_or(400);
        format!("{} {}px {}", weight, self.size, self.font)
    }

    pub fn weight(mut self, value: u32) -> Self {
        self.weight = Some(value);
        self
    }
}

impl Default for TextProperties {
    fn default() -> Self {
        Self {
            size: 20.0,
            font: String::from("Arial,sans-serif"),
            weight: None,
        }
    }
}
