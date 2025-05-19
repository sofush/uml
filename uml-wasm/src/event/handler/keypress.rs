use uml_common::{camera::Camera, elements::Class};

use crate::event::{Event, KeyboardEvent, Outcome};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KeypressHandler {
    x: i32,
    y: i32,
    pressed: bool,
}

impl KeypressHandler {
    pub fn handle(&mut self, event: &Event, camera: &Camera) -> Outcome {
        if let Event::Mouse(event) = event {
            self.x = event.x();
            self.y = event.y();
            return Outcome::None;
        }

        let Event::Keyboard(event) = event else {
            return Outcome::None;
        };

        if let KeyboardEvent::Up { key } = event {
            if key == "a" && self.pressed {
                self.pressed = false;
            }

            return Outcome::None;
        }

        let KeyboardEvent::Down { key } = event else {
            return Outcome::None;
        };

        if key != "a" || self.pressed {
            return Outcome::None;
        }

        self.pressed = true;
        let x = self.x + camera.x() as i32;
        let y = self.y + camera.y() as i32;
        let class = Class::new(x, y, None, None, Some(3));
        Outcome::AddElement(class.into())
    }
}
