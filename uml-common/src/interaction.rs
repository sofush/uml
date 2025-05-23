use crate::prompt::{Prompt, PromptResponse};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct InteractionState {
    hover: bool,
}

impl InteractionState {
    pub fn set_hover(&mut self, value: bool) {
        self.hover = value;
    }

    pub fn is_hovered(&self) -> bool {
        self.hover
    }
}

#[allow(unused_variables)]
pub trait Interactive {
    fn get_interaction(&self) -> InteractionState;
    fn get_interaction_mut(&mut self) -> &mut InteractionState;

    fn adjust_position(&mut self, delta_x: i32, delta_y: i32);

    fn set_position(&mut self, x: i32, y: i32) {}

    fn click(&mut self, x: i32, y: i32) -> Option<Prompt> {
        None
    }

    fn prompt(&mut self, response: PromptResponse) {}

    fn hover_enter(&mut self) {
        self.get_interaction_mut().set_hover(true);
    }

    fn hover_leave(&mut self) {
        self.get_interaction_mut().set_hover(false);
    }

    fn is_hovered(&self) -> bool {
        self.get_interaction().is_hovered()
    }
}
