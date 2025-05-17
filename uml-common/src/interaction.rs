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

pub trait Interactive {
    fn get_interaction(&self) -> InteractionState;
    fn get_interaction_mut(&mut self) -> &mut InteractionState;

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
