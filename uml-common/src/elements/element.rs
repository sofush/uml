use super::rectangle::Rectangle;
use crate::{canvas::Canvas, drawable::Drawable};

#[derive(Clone)]
pub enum Element {
    Rectangle(Rectangle),
}

impl Drawable for Element {
    fn draw(&self, canvas: &impl Canvas) {
        match self {
            Element::Rectangle(rectangle) => rectangle.draw(canvas),
        }
    }
}
