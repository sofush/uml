use crate::{camera::Camera, elements::Rectangle};

pub trait Canvas {
    #[allow(unused_variables)]
    fn draw_rectangle(&self, rect: Rectangle, camera: &Camera);
}
