use crate::elements::Element;

#[derive(Clone)]
pub struct Document {
    drawables: Vec<Element>,
}

impl Document {
    pub fn elements(&self) -> &Vec<Element> {
        &self.drawables
    }
}
