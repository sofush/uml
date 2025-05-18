use serde::{Deserialize, Serialize};

use super::{Label, rectangle::Rectangle};
use crate::{
    camera::Camera,
    canvas::Canvas,
    drawable::Drawable,
    id::Id,
    interaction::{InteractionState, Interactive},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum ElementType {
    Rectangle(Rectangle),
    Label(Label),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    #[serde(skip)]
    id: Id,
    inner: ElementType,
}

impl Element {
    pub fn cursor_intersects(&self, cursor_pos: (i32, i32)) -> bool {
        let cx = cursor_pos.0;
        let cy = cursor_pos.1;

        let (l, t, r, b) = match self.inner {
            ElementType::Rectangle(r) => (
                r.x(),
                r.y(),
                r.x() + r.width() as i32,
                r.y() + r.height() as i32,
            ),
            ElementType::Label(_) => {
                return false;
            }
        };

        cx >= l && cx <= r && cy >= t && cy <= b
    }

    pub fn as_interactive(&self) -> &dyn Interactive {
        match &self.inner {
            ElementType::Rectangle(rectangle) => rectangle,
            ElementType::Label(label) => label,
        }
    }

    pub fn as_interactive_mut(&mut self) -> &mut dyn Interactive {
        match &mut self.inner {
            ElementType::Rectangle(rectangle) => rectangle,
            ElementType::Label(label) => label,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }
}

impl Drawable for Element {
    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        match &self.inner {
            ElementType::Rectangle(rectangle) => rectangle.draw(canvas, camera),
            ElementType::Label(label) => label.draw(canvas, camera),
        }
    }
}

impl Interactive for Element {
    fn get_interaction(&self) -> InteractionState {
        match &self.inner {
            ElementType::Rectangle(rectangle) => rectangle.get_interaction(),
            ElementType::Label(label) => label.get_interaction(),
        }
    }

    fn get_interaction_mut(&mut self) -> &mut InteractionState {
        match &mut self.inner {
            ElementType::Rectangle(rectangle) => {
                rectangle.get_interaction_mut()
            }
            ElementType::Label(label) => label.get_interaction_mut(),
        }
    }

    fn adjust_position(&mut self, delta_x: i32, delta_y: i32) {
        match &mut self.inner {
            ElementType::Rectangle(rectangle) => {
                rectangle.adjust_position(delta_x, delta_y)
            }
            ElementType::Label(label) => {
                label.adjust_position(delta_x, delta_y)
            }
        }
    }
}

impl From<Rectangle> for Element {
    fn from(value: Rectangle) -> Self {
        Self {
            id: Id::default(),
            inner: ElementType::Rectangle(value),
        }
    }
}

impl From<Label> for Element {
    fn from(value: Label) -> Self {
        Self {
            id: Id::default(),
            inner: ElementType::Label(value),
        }
    }
}
