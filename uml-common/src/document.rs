use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    drawable::Drawable,
    elements::{Element, Rectangle},
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Document {
    elements: Vec<Element>,
    color: Color,
    #[serde(skip)]
    synchronized: bool,
}

impl Document {
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn add_element(&mut self, el: impl Into<Element>) {
        self.synchronized = false;
        self.elements.push(el.into());
    }

    pub fn synchronized(&self) -> bool {
        self.synchronized
    }

    pub fn assume_sync(&mut self) {
        self.synchronized = true;
    }

    pub fn draw(
        &self,
        canvas: &impl Canvas,
        camera: &Camera,
        cursor_pos: (i32, i32),
    ) {
        let clear_rect: Element = Rectangle::new(
            i32::MIN,
            i32::MIN,
            u32::MAX,
            u32::MAX,
            self.color,
            None,
        )
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
                    Rectangle::new(x as _, y as _, SIZE, SIZE, dot_color, None);
                rect.draw_fixed(canvas);
            }
        }

        for el in &self.elements {
            if el.cursor_intersects(cursor_pos) {
                let color = Color::Rgb {
                    red: 255,
                    green: 0,
                    blue: 0,
                };
                let x = 50.0;
                let y = 50.0;
                let rect = Rectangle::new(x as _, y as _, 50, 50, color, None);
                rect.draw_fixed(canvas);
            }

            el.draw(canvas, camera);
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            color: Color::Rgb {
                red: 240,
                green: 240,
                blue: 240,
            },
            elements: Default::default(),
            synchronized: true,
        }
    }
}
