use crate::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    drawable::Drawable,
    interaction::{Interactable, InteractionState},
};

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: Color,
    interaction_state: InteractionState,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Drawable for Rectangle {
    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        canvas.draw_rectangle(*self, camera);
    }
}

impl Interactable for Rectangle {
    fn interaction_state(&self) -> InteractionState {
        self.interaction_state
    }

    fn interaction_state_mut(
        &mut self,
    ) -> &mut crate::interaction::InteractionState {
        &mut self.interaction_state
    }
}
