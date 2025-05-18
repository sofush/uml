use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    drawable::Drawable,
    interaction::{InteractionState, Interactive},
};

use super::TextProperties;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    x: i32,
    y: i32,
    text: String,
    properties: TextProperties,
    color: Color,
    #[serde(skip)]
    interaction_state: InteractionState,
}

impl Label {
    pub fn new(
        x: i32,
        y: i32,
        text: impl Into<String>,
        props: TextProperties,
        color: Color,
    ) -> Self {
        Self {
            x,
            y,
            text: text.into(),
            properties: props,
            color,
            interaction_state: InteractionState::default(),
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn props(&self) -> &TextProperties {
        &self.properties
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Drawable for Label {
    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        canvas.draw_text(self, camera);
    }
}

impl Interactive for Label {
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
