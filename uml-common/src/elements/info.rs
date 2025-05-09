use crate::{
    camera::{self, Camera},
    canvas::Canvas,
    color::BLACK,
    drawable::Drawable,
};

use super::{Rectangle, TextProperties};

#[derive(Debug)]
pub struct Info {
    text: String,
    props: TextProperties,
}

impl Info {
    pub fn new(text: String, props: TextProperties) -> Self {
        Self { text, props }
    }
}

impl Drawable for Info {
    fn draw(&self, canvas: &impl Canvas, _: &Camera) {
        canvas.measure_text(&self.text, &self.props);
        let bg = Rectangle::new(0, 0, 50, 50, BLACK);
        bg.draw_fixed(canvas);
    }
}
