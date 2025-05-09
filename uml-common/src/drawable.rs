use crate::{camera::Camera, canvas::Canvas};

pub trait Drawable {
    #[allow(unused_variables)]
    fn draw(&self, canvas: &impl Canvas, camera: &Camera);

    #[allow(unused_variables)]
    fn draw_fixed(&self, canvas: &impl Canvas) {
        self.draw(canvas, &Camera::default());
    }
}
