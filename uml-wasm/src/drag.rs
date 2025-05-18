use uml_common::id::Id;

#[derive(Debug, Clone, Copy)]
pub enum DragState {
    None,
    Camera,
    Element { id: Id },
}

impl Default for DragState {
    fn default() -> Self {
        Self::None
    }
}
