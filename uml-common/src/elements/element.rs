use super::{Label, rectangle::Rectangle};
use crate::{camera::Camera, canvas::Canvas, drawable::Drawable};

#[derive(Clone)]
pub enum Element {
    Rectangle(Rectangle),
    Label(Label),
}

impl Drawable for Element {
    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        match self {
            Element::Rectangle(rectangle) => rectangle.draw(canvas, camera),
            Element::Label(label) => label.draw(canvas, camera),
        }
    }
}
