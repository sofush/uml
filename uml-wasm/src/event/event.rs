use std::fmt::Display;

use uml_common::{id::Id, prompt::PromptResponse};

use crate::{mouse_button::MouseButton, wsclient::WsEvent};

#[derive(Debug, Clone)]
pub enum MouseEvent {
    Down { button: MouseButton, x: i32, y: i32 },
    Up { button: MouseButton, x: i32, y: i32 },
    Move { x: i32, y: i32 },
    Out { x: i32, y: i32 },
    Enter { x: i32, y: i32 },
}

#[derive(Debug, Clone)]
pub enum KeyboardEvent {
    Down { key: String },
    Up { key: String },
}

#[derive(Debug, Clone)]
pub enum Event {
    Resize,
    Initialize,
    Redraw,
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    WebSocket(WsEvent),
    PromptResponse {
        element_id: Id,
        response: PromptResponse,
    },
}

impl MouseEvent {
    pub fn x(&self) -> i32 {
        match *self {
            MouseEvent::Down { x, .. } => x,
            MouseEvent::Up { x, .. } => x,
            MouseEvent::Move { x, .. } => x,
            MouseEvent::Out { x, .. } => x,
            MouseEvent::Enter { x, .. } => x,
        }
    }

    pub fn y(&self) -> i32 {
        match *self {
            MouseEvent::Down { y, .. } => y,
            MouseEvent::Up { y, .. } => y,
            MouseEvent::Move { y, .. } => y,
            MouseEvent::Out { y, .. } => y,
            MouseEvent::Enter { y, .. } => y,
        }
    }

    pub fn button(&self) -> Option<MouseButton> {
        match *self {
            MouseEvent::Down { button, .. } => button,
            MouseEvent::Up { button, .. } => button,
            _ => return None,
        }
        .into()
    }
}

impl KeyboardEvent {
    pub fn key(&self) -> &str {
        match self {
            KeyboardEvent::Down { key } => key,
            KeyboardEvent::Up { key } => key,
        }
    }
}

impl Display for MouseEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MouseEvent::Down { button, x, y } => {
                f.write_fmt(format_args!("MouseDown({button:?}, {x}, {y})"))
            }
            MouseEvent::Up { button, x, y } => {
                f.write_fmt(format_args!("MouseUp({button:?}, {x}, {y})"))
            }
            MouseEvent::Move { x, y } => {
                f.write_fmt(format_args!("MouseMove({x}, {y})"))
            }
            MouseEvent::Out { x, y } => {
                f.write_fmt(format_args!("MouseOut({x}, {y})"))
            }
            MouseEvent::Enter { x, y } => {
                f.write_fmt(format_args!("MouseEnter({x}, {y})"))
            }
        }
    }
}

impl Display for KeyboardEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyboardEvent::Down { key } => {
                f.write_fmt(format_args!("KeyDown(\"{key}\")"))
            }
            KeyboardEvent::Up { key } => {
                f.write_fmt(format_args!("KeyUp(\"{key}\")"))
            }
        }
    }
}

impl From<MouseEvent> for Event {
    fn from(value: MouseEvent) -> Self {
        Event::Mouse(value)
    }
}

impl From<KeyboardEvent> for Event {
    fn from(value: KeyboardEvent) -> Self {
        Event::Keyboard(value)
    }
}
