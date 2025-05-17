use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    drawable::Drawable,
    interaction::{InteractionState, Interactive},
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: Color,
    radius: Option<u32>,
    #[serde(skip)]
    interaction_state: InteractionState,
}

impl Rectangle {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color: Color,
        radius: Option<u32>,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
            radius,
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
}

impl Drawable for Rectangle {
    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        let mut copy = *self;

        if self.is_hovered() {
            copy.color = Color::Rgb {
                red: 255,
                green: 0,
                blue: 0,
            }
        }

        canvas.draw_rectangle(copy, camera);
    }
}

impl Interactive for Rectangle {
    fn get_interaction(&self) -> InteractionState {
        self.interaction_state
    }

    fn get_interaction_mut(&mut self) -> &mut InteractionState {
        &mut self.interaction_state
    }
}
