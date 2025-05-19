use crate::{camera::Camera, canvas::Canvas};

#[allow(unused_variables)]
pub trait Drawable {
    fn initalize(&mut self, canvas: &impl Canvas) {}

    fn draw(&self, canvas: &impl Canvas, camera: &Camera);

    fn draw_fixed(&self, canvas: &impl Canvas) {
        self.draw(canvas, &Camera::default());
    }
}
