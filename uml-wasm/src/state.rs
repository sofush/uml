use crate::{event::Event, html_canvas::HtmlCanvas, mouse_button::MouseButton};
use std::{cell::RefCell, collections::HashSet, thread_local};
use uml_common::{
    camera::Camera,
    color::BLACK,
    document::Document,
    drawable::Drawable,
    elements::{Info, Rectangle, TextProperties},
};

thread_local! {
    pub static SHARED_STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

const TRANSLATE_KEY: &str = " ";

pub struct State {
    document: Document,
    canvas: HtmlCanvas,
    camera: Camera,
    keys_pressed: HashSet<String>,
    cursor_pos: (i32, i32),
    mouse_buttons: HashSet<MouseButton>,
    translate_camera: bool,
}

impl State {
    pub fn new(document: Document, canvas: HtmlCanvas) -> Self {
        Self {
            document,
            canvas,
            camera: Camera::default(),
            keys_pressed: HashSet::new(),
            mouse_buttons: HashSet::new(),
            cursor_pos: (0, 0),
            translate_camera: false,
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        log::trace!("Handling event: {event}...");

        match event.clone() {
            Event::MouseDown { button, .. } => {
                self.mouse_buttons.insert(button);
            }
            Event::MouseUp { button, .. } => {
                self.mouse_buttons.remove(&button);
            }
            Event::MouseMove { x, y } => {
                let delta_x = x - self.cursor_pos.0;
                let delta_y = y - self.cursor_pos.1;

                if self.translate_camera {
                    self.camera.translate(-delta_x as f64, -delta_y as f64);
                    log::trace!(
                        "Camera state after translate: {:?}",
                        self.camera
                    );
                }

                self.cursor_pos = (x, y);
            }
            Event::KeyDown { key } => {
                self.keys_pressed.insert(key);
            }
            Event::KeyUp { key } => {
                self.keys_pressed.remove(&key);
            }
            Event::Resize => self.canvas.update_size(),
            Event::Redraw => (),
        };

        {
            let button = self.mouse_buttons.contains(&MouseButton::Left);
            let key = self.keys_pressed.contains(TRANSLATE_KEY);
            self.translate_camera = button && key;
        }

        if let Event::MouseDown { x, y, .. } = event {
            if !self.translate_camera {
                let x = x + self.camera.x() as i32;
                let y = y + self.camera.y() as i32;
                // let props = TextProperties::new(50.0, "Arial");
                // let label = Label::new(x, y, "hello", props, BLACK);
                // self.document.elements_mut().push(label.into());
                let rect = Rectangle::new(x, y, 100, 100, BLACK);
                self.document.elements_mut().push(rect.into());
            }
        }

        let cursor_pos = (
            self.cursor_pos.0 + self.camera.x() as i32,
            self.cursor_pos.1 + self.camera.y() as i32,
        );

        self.document.draw(&self.canvas, &self.camera, cursor_pos);

        if self.translate_camera {
            let props = TextProperties::new(20.0, "Arial");
            let text = format!("{}x {}y", self.camera.x(), self.camera.y());
            let info = Info::new(text, props);
            info.draw_fixed(&self.canvas);
        }
    }
}
