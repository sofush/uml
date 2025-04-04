use std::fmt::Display;

use crate::mouse_button::MouseButton;

#[derive(Clone)]
pub enum Event {
    Resize,
    MouseDown { button: MouseButton, x: i32, y: i32 },
    MouseUp { button: MouseButton, x: i32, y: i32 },
    MouseMove { x: i32, y: i32 },
    KeyDown { key: String },
    KeyUp { key: String },
    Redraw,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Resize => f.write_str("Resize"),
            Event::MouseDown { button, x, y } => {
                f.write_fmt(format_args!("MouseDown({button:?}, {x}, {y})"))
            }
            Event::MouseUp { button, x, y } => {
                f.write_fmt(format_args!("MouseUp({button:?}, {x}, {y})"))
            }
            Event::MouseMove { x, y } => {
                f.write_fmt(format_args!("MouseMove({x}, {y})"))
            }
            Event::KeyDown { key } => {
                f.write_fmt(format_args!("KeyDown(\"{key}\")"))
            }
            Event::KeyUp { key } => {
                f.write_fmt(format_args!("KeyUp(\"{key}\")"))
            }
            Event::Redraw => f.write_str("Redraw"),
        }
    }
}
