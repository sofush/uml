use crate::draw_context::Canvas;

pub(crate) trait Drawable {
    #[allow(unused_variables)]
    fn draw(&self, canvas: &impl Canvas) {}
}
