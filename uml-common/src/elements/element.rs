use super::rectangle::Rectangle;
use crate::{draw_context::Canvas, drawable::Drawable};

#[derive(Clone)]
pub enum Element {
    Rectangle(Rectangle),
}

impl Drawable for Element {
    fn draw(&self, canvas: &impl Canvas) {
        match self {
            Element::Rectangle(rectangle) => canvas.draw_rectangle(*rectangle),
        }
    }
}
