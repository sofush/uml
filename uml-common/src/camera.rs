#[derive(Debug, Default)]
pub struct Camera {
    x: f64,
    y: f64,
}

impl Camera {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
}
