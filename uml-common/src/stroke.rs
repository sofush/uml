use serde::{Deserialize, Serialize};

use crate::color::Color;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Stroke {
    width: u32,
    color: Color,
}

impl Stroke {
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub const fn new(width: u32, color: Color) -> Self {
        Self { width, color }
    }
}
