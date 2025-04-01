use std::fmt::Display;

#[derive(Clone)]
pub enum Event {
    Resize,
    MouseDown { x: u32, y: u32 },
    MouseUp { x: u32, y: u32 },
    MouseMove { x: u32, y: u32 },
    KeyDown { key: String },
    KeyUp { key: String },
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Resize => f.write_fmt(format_args!("Resize")),
            Event::MouseDown { x, y } => {
                f.write_fmt(format_args!("MouseDown({x}, {y})"))
            }
            Event::MouseUp { x, y } => {
                f.write_fmt(format_args!("MouseUp({x}, {y})"))
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
        }
    }
}
