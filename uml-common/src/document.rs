use crate::{
    camera::Camera,
    canvas::Canvas,
    color::WHITE,
    drawable::Drawable,
    elements::{Element, Rectangle},
};

#[derive(Clone, Default)]
pub struct Document {
    drawables: Vec<Element>,
}

impl Document {
    pub fn elements(&mut self) -> &mut Vec<Element> {
        &mut self.drawables
    }
}

impl Drawable for Document {
    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        let clear_rect: Element =
            Rectangle::new(i32::MIN, i32::MIN, u32::MAX, u32::MAX, WHITE)
                .into();
        clear_rect.draw(canvas, camera);

        for drawable in &self.drawables {
            drawable.draw(canvas, camera);
        }
    }
}
