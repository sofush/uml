use std::u8;

#[derive(Clone, Copy)]
pub enum Color {
    Rgb { red: u8, green: u8, blue: u8 },
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Rgb { red, green, blue } => {
                format!("rgb({red} {green} {blue})")
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
