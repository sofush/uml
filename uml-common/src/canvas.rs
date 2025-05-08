use crate::elements::Rectangle;

pub trait Canvas {
    #[allow(unused_variables)]
    fn draw_rectangle(&self, rect: Rectangle);
}
