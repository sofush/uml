use uml_common::{camera::Camera, elements::Element, id::Id};

use crate::{
    event::{
        Event, KeyboardEvent, MouseEvent, Outcome, cursor_style::CursorStyle,
    },
    mouse_button::MouseButton,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DragState {
    #[default]
    None,
    Camera,
    PressingElement {
        id: Id,
    },
    DraggingElement {
        id: Id,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DragHandler {
    state: DragState,
    cursor: CursorStyle,

    translate_key: bool,
    left_button: bool,

    x: i32,
    y: i32,
}

impl DragHandler {
    pub fn handle(
        &mut self,
        event: &Event,
        elements: &[Element],
        camera: Camera,
    ) -> Vec<Outcome> {
        let old_state = *self;

        match event {
            Event::Mouse(ev) => self.handle_mouse_event(ev),
            Event::Keyboard(ev) => self.handle_keyboard_event(ev),
            _ => return vec![Outcome::None],
        }

        let primary_outcome = self.update(old_state, elements, camera);
        let mut outcomes = vec![primary_outcome];

        let is_translating = matches!(self.state, DragState::Camera);
        if self.state != old_state.state || is_translating {
            outcomes.push(Outcome::UpdateInfo {
                visible: is_translating,
            });
        }

        self.cursor = self.get_cursor();

        if self.cursor != old_state.get_cursor() {
            outcomes.push(Outcome::CursorStyle(self.cursor));
        }

        outcomes
    }

    fn handle_keyboard_event(&mut self, event: &KeyboardEvent) {
        if event.key() != " " {
            return;
        }

        self.translate_key = match event {
            KeyboardEvent::Down { .. } => true,
            KeyboardEvent::Up { .. } => false,
        };
    }

    fn handle_mouse_event(&mut self, event: &MouseEvent) {
        if event.button() == Some(MouseButton::Left) {
            self.left_button = match event {
                MouseEvent::Down { .. } => true,
                MouseEvent::Up { .. } => false,
                _ => return,
            };
        }

        self.x = event.x();
        self.y = event.y();
    }

    fn update(
        &mut self,
        old_state: DragHandler,
        elements: &[Element],
        camera: Camera,
    ) -> Outcome {
        let delta_x = self.x - old_state.x;
        let delta_y = self.y - old_state.y;

        match self.state {
            DragState::None => {
                if self.translate_key {
                    if self.left_button {
                        self.state = DragState::Camera;
                    }

                    return Outcome::None;
                }

                if !self.left_button {
                    return Outcome::None;
                }

                let x = self.x + camera.x() as i32;
                let y = self.y + camera.y() as i32;

                if let Some(el) =
                    elements.iter().rev().find(|e| e.cursor_intersects(x, y))
                {
                    self.state = DragState::PressingElement { id: el.id() };
                }

                Outcome::None
            }
            DragState::Camera => {
                if !self.left_button || !self.translate_key {
                    self.state = DragState::None;
                    return Outcome::None;
                }

                Outcome::Translate {
                    x: -delta_x,
                    y: -delta_y,
                }
            }
            DragState::PressingElement { id } => {
                if self.x != old_state.x || self.y != old_state.y {
                    self.state = DragState::DraggingElement { id };
                    Outcome::MoveElement {
                        id,
                        x: delta_x,
                        y: delta_y,
                    }
                } else if !self.left_button {
                    self.state = DragState::None;
                    Outcome::ClickElement {
                        id,
                        x: self.x + camera.x() as i32,
                        y: self.y + camera.y() as i32,
                    }
                } else {
                    Outcome::None
                }
            }
            DragState::DraggingElement { id } => {
                if !self.left_button {
                    self.state = DragState::None;
                    Outcome::None
                } else if delta_x != 0 || delta_y != 0 {
                    Outcome::MoveElement {
                        id,
                        x: delta_x,
                        y: delta_y,
                    }
                } else {
                    Outcome::None
                }
            }
        }
    }

    fn get_cursor(&self) -> CursorStyle {
        match self.state {
            DragState::None if self.translate_key => CursorStyle::Grab,
            DragState::Camera => CursorStyle::Grabbing,
            _ => CursorStyle::Default,
        }
    }
}
