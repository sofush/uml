use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    drawable::Drawable,
    elements::{Element, Info, Rectangle},
    interaction::Interactive,
};

#[derive(Clone, Debug, PartialEq, Default)]
struct LocalData {
    show_info: bool,
    info_element: Info,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Document {
    elements: Vec<Element>,
    color: Color,
    #[serde(skip)]
    local: LocalData,
}

impl Document {
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }

    pub fn add_element(&mut self, el: impl Into<Element>) {
        self.elements.push(el.into());
    }

    pub fn update_cursor(&mut self, x: i32, y: i32, visible: bool) {
        for el in &mut self.elements {
            match (visible, el.is_hovered(), el.cursor_intersects(x, y)) {
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
                let rect = Rectangle::new(
                    x as _, y as _, SIZE, SIZE, dot_color, None, None,
                );
                rect.draw_fixed(canvas);
            }
        }

        for element in &self.elements {
            element.draw(canvas, camera);
        }

        if self.local.show_info {
            self.local.info_element.draw_fixed(canvas);
        }
    }

    pub fn update_info(
        &mut self,
        visible: impl Into<Option<bool>>,
        text: String,
    ) {
        if let Some(visible) = visible.into() {
            self.local.show_info = visible;
        }
        self.local.info_element.set_text(text);
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
            local: LocalData {
                show_info: false,
                info_element: Info::default(),
            },
        }
    }
}
