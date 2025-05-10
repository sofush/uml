use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Color {
    Rgb { red: u8, green: u8, blue: u8 },
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Rgb { red, green, blue } => {
                f.write_fmt(format_args!("RGB({red} {green} {blue})"))
            }
        }
    }
}

pub static BLACK: Color = Color::Rgb {
    red: 0,
    green: 0,
    blue: 0,
};

pub static WHITE: Color = Color::Rgb {
    red: u8::MAX,
    green: u8::MAX,
    blue: u8::MAX,
};
