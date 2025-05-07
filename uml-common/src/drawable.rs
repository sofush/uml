use crate::draw_context::Canvas;

pub trait Drawable {
    #[allow(unused_variables)]
    fn draw(&self, canvas: &impl Canvas);
}
