use super::Element;
use crate::{camera::Camera, canvas::Canvas, color::Color, drawable::Drawable};

#[derive(Clone, Copy)]
pub struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: Color,
}

impl Rectangle {
    pub fn new(x: u32, y: u32, width: u32, height: u32, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl From<Rectangle> for Element {
    fn from(value: Rectangle) -> Self {
        Element::Rectangle(value)
    }
}

impl Drawable for Rectangle {
    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        canvas.draw_rectangle(*self, camera);
    }
}
