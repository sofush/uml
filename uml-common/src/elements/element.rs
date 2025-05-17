use serde::{Deserialize, Serialize};

use super::{Label, rectangle::Rectangle};
use crate::{
    camera::Camera,
    canvas::Canvas,
    drawable::Drawable,
    interaction::{InteractionState, Interactive},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Element {
    Rectangle(Rectangle),
    Label(Label),
}

impl Element {
    pub fn cursor_intersects(&self, cursor_pos: (i32, i32)) -> bool {
        let cx = cursor_pos.0;
        let cy = cursor_pos.1;

        let (l, t, r, b) = match self {
            Element::Rectangle(r) => (
                r.x(),
                r.y(),
                r.x() + r.width() as i32,
                r.y() + r.height() as i32,
            ),
            Element::Label(_) => {
                return false;
            }
        };

        cx >= l && cx <= r && cy >= t && cy <= b
    }
}

impl Drawable for Element {
    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        match self {
            Element::Rectangle(rectangle) => rectangle.draw(canvas, camera),
            Element::Label(label) => label.draw(canvas, camera),
        }
    }
}

impl Interactive for Element {
    fn get_interaction(&self) -> InteractionState {
        match self {
            Element::Rectangle(rectangle) => rectangle.get_interaction(),
            Element::Label(label) => label.get_interaction(),
        }
    }

    fn get_interaction_mut(&mut self) -> &mut InteractionState {
        match self {
            Element::Rectangle(rectangle) => rectangle.get_interaction_mut(),
            Element::Label(label) => label.get_interaction_mut(),
        }
    }
}

impl From<Rectangle> for Element {
    fn from(value: Rectangle) -> Self {
        Self::Rectangle(value)
    }
}

impl From<Label> for Element {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}
