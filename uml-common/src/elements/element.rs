use super::{Label, rectangle::Rectangle};
use crate::{
    camera::Camera,
    canvas::Canvas,
    drawable::Drawable,
    interaction::{Interactable, InteractionState},
};

#[derive(Debug, Clone, PartialEq)]
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

impl Interactable for Element {
    fn interaction_state(&self) -> InteractionState {
        match self {
            Element::Rectangle(rectangle) => rectangle.interaction_state(),
            Element::Label(label) => label.interaction_state(),
        }
    }

    fn interaction_state_mut(&mut self) -> &mut InteractionState {
        match self {
            Element::Rectangle(rectangle) => rectangle.interaction_state_mut(),
            Element::Label(label) => label.interaction_state_mut(),
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
