use crate::{
    camera::Camera, canvas::Canvas, color::BLACK, drawable::Drawable,
    stroke::Stroke,
};

use super::{Label, Rectangle, TextProperties};

#[derive(Debug, Clone, PartialEq)]
pub struct Info {
    text: String,
    props: TextProperties,
}

impl Info {
    pub fn new(text: String, props: TextProperties) -> Self {
        Self { text, props }
    }

    pub fn set_text(&mut self, value: String) {
        self.text = value;
    }
}

impl Drawable for Info {
    fn draw(&self, canvas: &impl Canvas, _: &Camera) {
        const MARGIN: i32 = 30;
        const PADDING: i32 = 10;

        let text = self.text.clone();
        let props = self.props.clone();
        let Some(measurement) = canvas.measure_text(&text, &props) else {
            log::error!("Could not measure text for info element.");
            return;
        };
        let (width, height) =
            (measurement.width() as i32, measurement.height() as i32);

        let bg = Rectangle::new(
            MARGIN,
            MARGIN,
            (width + PADDING * 2) as u32,
            (height + PADDING * 2) as u32,
            crate::color::Color::Rgb {
                red: 255,
                green: 255,
                blue: 0,
            },
            None,
            Some(Stroke::new(1, BLACK)),
        );
        bg.draw_fixed(canvas);

        let label = Label::new(
            MARGIN + PADDING,
            MARGIN + PADDING + height,
            self.text.clone(),
            self.props.clone(),
            BLACK,
        );
        label.draw_fixed(canvas);
    }
}

impl Default for Info {
    fn default() -> Self {
        Self {
            text: String::new(),
            props: TextProperties::new(20.0, "Serif"),
        }
    }
}
