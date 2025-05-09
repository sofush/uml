use crate::{camera::Camera, canvas::Canvas, color::Color, drawable::Drawable};

use super::{Element, TextProperties};

#[derive(Debug, Clone)]
pub struct Label {
    x: i32,
    y: i32,
    text: String,
    properties: TextProperties,
    color: Color,
}

impl Label {
    pub fn new(
        x: i32,
        y: i32,
        text: impl Into<String>,
        props: TextProperties,
        color: Color,
    ) -> Self {
        Self {
            x,
            y,
            text: text.into(),
            properties: props,
            color,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn props(&self) -> &TextProperties {
        &self.properties
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl From<Label> for Element {
    fn from(value: Label) -> Self {
        Element::Label(value)
    }
}

impl Drawable for Label {
    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        canvas.draw_text(&self, camera);
    }
}
