#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct InteractionState {
    hover: bool,
}

impl InteractionState {
    pub fn hover(&self) -> bool {
        self.hover
    }

    pub fn set_hover(&mut self, hover: bool) {
        self.hover = hover;
    }
}

pub trait Interactable {
    fn interaction_state(&self) -> InteractionState;
    fn interaction_state_mut(&mut self) -> &mut InteractionState;
}
