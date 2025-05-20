use serde::{Deserialize, Serialize};

use super::{Class, Label, rectangle::Rectangle};
use crate::{
    camera::Camera,
    canvas::Canvas,
    drawable::Drawable,
    id::Id,
    interaction::{InteractionState, Interactive},
    prompt::{Prompt, PromptResponse},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum ElementType {
    Rectangle(Rectangle),
    Label(Label),
    Class(Class),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    #[serde(skip)]
    id: Id,
    inner: ElementType,
}

impl Element {
    pub fn x(&self) -> i32 {
        match &self.inner {
            ElementType::Rectangle(rectangle) => rectangle.x(),
            ElementType::Label(label) => label.x(),
            ElementType::Class(class) => class.x(),
        }
    }

    pub fn y(&self) -> i32 {
        match &self.inner {
            ElementType::Rectangle(rectangle) => rectangle.y(),
            ElementType::Label(label) => label.y(),
            ElementType::Class(class) => class.y(),
        }
    }

    pub fn cursor_intersects(&self, x: i32, y: i32) -> bool {
        let (l, t, r, b) = match &self.inner {
            ElementType::Rectangle(r) => (
                r.x(),
                r.y(),
                r.x() + r.width() as i32,
                r.y() + r.height() as i32,
            ),
            ElementType::Label(l) => (
                l.x(),
                l.y(),
                l.x() + l.width().unwrap_or(0) as i32,
                l.y() + l.height().unwrap_or(0) as i32,
            ),
            ElementType::Class(c) => (
                c.x(),
                c.y(),
                c.x() + c.width().unwrap_or(0) as i32,
                c.y() + c.height().unwrap_or(0) as i32,
            ),
        };

        x >= l && x <= r && y >= t && y <= b
    }

    pub fn as_interactive(&self) -> &dyn Interactive {
        match &self.inner {
            ElementType::Rectangle(rectangle) => rectangle,
            ElementType::Label(label) => label,
            ElementType::Class(class) => class,
        }
    }

    pub fn as_interactive_mut(&mut self) -> &mut dyn Interactive {
        match &mut self.inner {
            ElementType::Rectangle(rectangle) => rectangle,
            ElementType::Label(label) => label,
            ElementType::Class(class) => class,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }
}

impl Drawable for Element {
    fn initalize(&mut self, canvas: &impl Canvas) {
        match &mut self.inner {
            ElementType::Rectangle(rectangle) => rectangle.initalize(canvas),
            ElementType::Label(label) => label.initalize(canvas),
            ElementType::Class(class) => class.initalize(canvas),
        }
    }

    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        match &self.inner {
            ElementType::Rectangle(rectangle) => rectangle.draw(canvas, camera),
            ElementType::Label(label) => label.draw(canvas, camera),
            ElementType::Class(class) => class.draw(canvas, camera),
        }
    }
}

impl Interactive for Element {
    fn get_interaction(&self) -> InteractionState {
        match &self.inner {
            ElementType::Rectangle(rectangle) => rectangle.get_interaction(),
            ElementType::Label(label) => label.get_interaction(),
            ElementType::Class(class) => class.get_interaction(),
        }
    }

    fn get_interaction_mut(&mut self) -> &mut InteractionState {
        match &mut self.inner {
            ElementType::Rectangle(rectangle) => {
                rectangle.get_interaction_mut()
            }
            ElementType::Label(label) => label.get_interaction_mut(),
            ElementType::Class(class) => class.get_interaction_mut(),
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
            ElementType::Class(class) => {
                class.adjust_position(delta_x, delta_y)
            }
        }
    }

    fn click(&mut self, x: i32, y: i32) -> Option<Prompt> {
        match &mut self.inner {
            ElementType::Rectangle(rectangle) => rectangle.click(x, y),
            ElementType::Label(label) => label.click(x, y),
            ElementType::Class(class) => class.click(x, y),
        }
    }

    fn prompt(&mut self, response: PromptResponse) {
        match &mut self.inner {
            ElementType::Rectangle(rectangle) => rectangle.prompt(response),
            ElementType::Label(label) => label.prompt(response),
            ElementType::Class(class) => class.prompt(response),
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

impl From<Class> for Element {
    fn from(value: Class) -> Self {
        Self {
            id: Id::default(),
            inner: ElementType::Class(value),
        }
    }
}
