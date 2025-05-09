use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy)]
pub struct Size<T>
where
    T: Display + Debug + Copy,
{
    height: T,
    width: T,
}

impl<T> Size<T>
where
    T: Display + Debug + Copy,
{
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn height(&self) -> T {
        self.height
    }

    pub fn width(&self) -> T {
        self.width
    }
}

impl<T> Display for Size<T>
where
    T: Display + Debug + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Size(height {}, width {})",
            self.height, self.width
        ))
    }
}
