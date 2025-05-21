use crate::{
    camera::Camera,
    canvas::Canvas,
    color::{BLACK, Color},
    drawable::Drawable,
    stroke::Stroke,
};

use super::{Label, Rectangle, TextProperties};

const MARGIN: i32 = 30;
const PADDING: i32 = 10;

const TEXT_COLOR: Color = BLACK;
const BACKGROUND_COLOR: Color = const {
    Color::Rgb {
        red: 255,
        green: 255,
        blue: 0,
    }
};

const STROKE: Stroke = const { Stroke::new(2, BLACK) };
const BORDER_RADIUS: u32 = 2;

#[derive(Debug, Clone, PartialEq)]
pub struct Info {
    text: String,
    props: TextProperties,
}

impl Info {
    pub fn set_text(&mut self, value: String) {
        self.text = value;
    }
}

impl Drawable for Info {
    fn draw(&self, canvas: &impl Canvas, _: &Camera) {
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
            BACKGROUND_COLOR,
            Some(BORDER_RADIUS),
            Some(STROKE),
        );
        bg.draw_fixed(canvas);

        let label = Label::new(
            MARGIN + PADDING,
            MARGIN + PADDING + height,
            self.text.clone(),
            self.props.clone(),
            TEXT_COLOR,
        );
        label.draw_fixed(canvas);
    }
}

impl Default for Info {
    fn default() -> Self {
        Self {
            text: String::new(),
            props: TextProperties::new(20.0, "Arial,sans-serif"),
        }
    }
}
