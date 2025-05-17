use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    drawable::Drawable,
    elements::{Element, Rectangle},
    interaction::Interactive,
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

    pub fn update_cursor(&mut self, cursor_pos: (i32, i32), visible: bool) {
        for el in &mut self.elements {
            match (visible, el.is_hovered(), el.cursor_intersects(cursor_pos)) {
                (true, true, false) => el.hover_leave(),
                (true, false, true) => el.hover_enter(),
                (false, true, _) => el.hover_leave(),
                _ => (),
            }
        }
    }

    pub fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
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

        for element in &self.elements {
            element.draw(canvas, camera);
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
