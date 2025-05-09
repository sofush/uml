use crate::{
    camera::Camera,
    canvas::Canvas,
    color::{Color, WHITE},
    drawable::Drawable,
    elements::{Element, Rectangle},
};

#[derive(Clone, Default)]
pub struct Document {
    elements: Vec<Element>,
}

impl Document {
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }

impl Drawable for Document {
    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        let clear_rect: Element =
            Rectangle::new(i32::MIN, i32::MIN, u32::MAX, u32::MAX, WHITE)
                .into();
        clear_rect.draw(canvas, camera);

        const SIZE: u32 = 2;
        const SPACE: f64 = 75.0;
        let offx = SPACE - (camera.x() % SPACE);
        let offy = SPACE - (camera.y() % SPACE);
        let dot_color = Color::Rgb {
            red: 170,
            green: 170,
            blue: 170,
        };

        for row in -1..100 {
            for col in -1..100 {
                let x = (row as f64 * SPACE) + offx;
                let y = (col as f64 * SPACE) + offy;
                let rect =
                    Rectangle::new(x as _, y as _, SIZE, SIZE, dot_color);
                rect.draw_fixed(canvas);
            }
        }

        for drawable in &self.drawables {
            drawable.draw(canvas, camera);
        }
    }
}
