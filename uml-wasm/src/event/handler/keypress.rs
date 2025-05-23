use std::collections::HashSet;

use uml_common::{camera::Camera, elements::Class};

use crate::{
    dialog::SHARED_DIALOG,
    event::{Event, KeyboardEvent, Outcome},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KeypressHandler {
    x: i32,
    y: i32,
    keys: HashSet<String>,
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

        let key = match event {
            KeyboardEvent::Down { key } => {
                self.keys.insert(key.to_string());
                return Outcome::None;
            }
            KeyboardEvent::Up { key } => {
                if !self.keys.remove(key) {
                    return Outcome::None;
                }

                key.as_str()
            }
        };

        if SHARED_DIALOG.with_borrow(|d| d.is_active()) {
            if key == "Escape" {
                SHARED_DIALOG.with_borrow_mut(|d| {
                    d.deactivate();
                });
            }

            return Outcome::None;
        }

        match key {
            "a" => {
                let x = self.x + camera.x() as i32;
                let y = self.y + camera.y() as i32;
                let class =
                    Class::new(x, y, "Test class".into(), None, None, Some(3));
                Outcome::AddElement(class.into())
            }
            _ => Outcome::None,
        }
    }
}
