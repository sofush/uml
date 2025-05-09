use crate::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    drawable::Drawable,
    interaction::{Interactable, InteractionState},
};

use super::TextProperties;

#[derive(Debug, Clone)]
pub struct Label {
    x: i32,
    y: i32,
    text: String,
    properties: TextProperties,
    color: Color,
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

impl Interactable for Label {
    fn interaction_state(&self) -> InteractionState {
        self.interaction_state
    }

    fn interaction_state_mut(
        &mut self,
    ) -> &mut crate::interaction::InteractionState {
        &mut self.interaction_state
    }
}
