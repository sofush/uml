use crate::{draw_context::Canvas, drawable::Drawable, elements::Element};

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
    fn draw(&self, canvas: &impl Canvas) {
        for drawable in &self.drawables {
            drawable.draw(canvas);
        }
    }
}
