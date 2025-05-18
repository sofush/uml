use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    drawable::Drawable,
    interaction::{InteractionState, Interactive},
    stroke::Stroke,
};

use super::Rectangle;

const DEFAULT_COLOR: Color = const {
    Color::Rgb {
        red: 244,
        green: 244,
        blue: 244,
    }
};

const DEFAULT_STROKE: Stroke = const {
    Stroke::new(
        2,
        Color::Rgb {
            red: 210,
            green: 210,
            blue: 210,
        },
    )
};

const HIGHLIGHT_STROKE: Stroke = const {
    Stroke::new(
        2,
        Color::Rgb {
            red: 142,
            green: 202,
            blue: 230,
        },
    )
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Class {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: Color,
    radius: Option<u32>,
    stroke: Option<Stroke>,
    #[serde(skip)]
    interaction_state: InteractionState,
}

impl Class {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color: Option<Color>,
        stroke: Option<Stroke>,
        radius: Option<u32>,
    ) -> Self {
        let color = color.unwrap_or(DEFAULT_COLOR);

        Self {
            x,
            y,
            width,
            height,
            color,
            radius,
            stroke,
            interaction_state: InteractionState::default(),
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn radius(&self) -> Option<u32> {
        self.radius
    }

    pub fn stroke(&self) -> Option<Stroke> {
        self.stroke
    }
}

impl Drawable for Class {
    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        let stroke = if self.is_hovered() {
            HIGHLIGHT_STROKE
        } else {
            self.stroke.unwrap_or(DEFAULT_STROKE)
        };

        let rect = Rectangle::new(
            self.x,
            self.y,
            self.width,
            self.height,
            self.color,
            self.radius,
            Some(stroke),
        );

        rect.draw(canvas, camera);
    }
}

impl Interactive for Class {
    fn get_interaction(&self) -> InteractionState {
        self.interaction_state
    }

    fn get_interaction_mut(&mut self) -> &mut InteractionState {
        &mut self.interaction_state
    }

    fn adjust_position(&mut self, delta_x: i32, delta_y: i32) {
        self.x += delta_x;
        self.y += delta_y;
    }
}
