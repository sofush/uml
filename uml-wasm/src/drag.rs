use uml_common::id::Id;

#[derive(Debug, Clone, Copy)]
pub enum DragState {
    None,
    Camera,
    PressingElement { id: Id },
    DraggingElement { id: Id },
}

impl Default for DragState {
    fn default() -> Self {
        Self::None
    }
}
