use crate::{
    camera::Camera,
    elements::{Label, Rectangle, TextProperties},
    size::Size,
};

pub trait Canvas {
    fn draw_rectangle(&self, rect: Rectangle, camera: &Camera);
    fn draw_text(&self, label: &Label, camera: &Camera);
    fn measure_text(
        &self,
        text: &str,
        props: &TextProperties,
    ) -> Option<Size<f32>>;
}
