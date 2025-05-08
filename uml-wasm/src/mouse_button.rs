#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
}

impl TryFrom<i16> for MouseButton {
    type Error = &'static str;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let button = match value {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            3 => MouseButton::Back,
            4 => MouseButton::Forward,
            _ => return Err("Unrecognized value."),
        };

        Ok(button)
    }
}
