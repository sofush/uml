use uml_common::{document::Document, elements::Element, id::Id};

use super::cursor_style::CursorStyle;

/// The result of a strategy. It describes what work needs to be done post-eventhandling (e.g.
/// rendering, moving an element or translating the camera).
#[derive(Default, Clone, Debug, PartialEq)]
pub enum Outcome {
    /// No-op.
    #[default]
    None,
    /// Update local document.
    UpdateDocument(Document),
    /// Translate the camera. The coordinates are relative to the previous cursor position.
    Translate { x: i32, y: i32 },
    /// Move an element. The coordinates are relative to the previous cursor position.
    MoveElement { id: Id, x: i32, y: i32 },
    /// Click an element. The coordinates are relative to the document's origin (0, 0).
    ClickElement { id: Id, x: i32, y: i32 },
    /// Update hover flag of an element.
    HoverElement { id: Id, hovered: bool },
    /// Change the style of the cursor.
    CursorStyle(CursorStyle),
    /// Update `Info` element.
    UpdateInfo { visible: bool },
    /// Add an element to the document.
    AddElement(Element),
}
