use std::fmt::Display;

#[derive(Clone)]
pub enum Event {
    Click { x: u32, y: u32 },
    Resize,
    MouseMove { x: u32, y: u32 },
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Click { x, y } => {
                f.write_fmt(format_args!("Click({x}, {y})"))
            }
            Event::Resize => f.write_fmt(format_args!("Resize")),
            Event::MouseMove { x, y } => {
                f.write_fmt(format_args!("MouseMove({x}, {y})"))
            }
        }
    }
}
