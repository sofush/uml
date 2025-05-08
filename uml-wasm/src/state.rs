use crate::{
    camera::Camera, event::Event, html_canvas::HtmlCanvas,
    mouse_button::MouseButton,
};
use std::{cell::RefCell, collections::HashSet, thread_local};
use uml_common::{
    color::BLACK, document::Document, drawable::Drawable, elements::Rectangle,
};

thread_local! {
    pub static SHARED_STATE: RefCell<Option<State>> = RefCell::new(None);
}

const TRANSLATE_KEY: &'static str = " ";

pub struct State {
    document: Document,
    canvas: HtmlCanvas,
    camera: Camera,
    keys_pressed: HashSet<String>,
    mouse_x: u32,
    mouse_y: u32,
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
            mouse_x: 0,
            mouse_y: 0,
            translate_camera: false,
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        log::trace!("Handling event: {event}...");

        match event {
            Event::MouseDown { button, x, y } => {
                self.mouse_buttons.insert(button);
                let rect = Rectangle::new(x, y, 10, 10, BLACK);
                self.document.elements().push(rect.into());
            }
            Event::MouseUp { button, .. } => {
                self.mouse_buttons.remove(&button);
            }
            Event::MouseMove { x, y } => {
                let delta_x = x - self.mouse_x;
                let delta_y = y - self.mouse_y;

                if self.translate_camera {
                    self.camera.x += delta_x;
                    self.camera.y += delta_y;
                }
            }
            Event::KeyDown { key } => {
                self.keys_pressed.insert(key);
            }
            Event::KeyUp { key } => {
                self.keys_pressed.remove(&key);
            }
            Event::Resize => self.canvas.update_size(),
        };

        {
            let button = self.mouse_buttons.contains(&MouseButton::Left) as u32;
            let key = self.keys_pressed.contains(TRANSLATE_KEY) as u32;
            let translate = self.translate_camera as u32;

            if button + key + translate >= 2 {
                self.translate_camera = true;
            } else if self.translate_camera {
                self.translate_camera = false;
            }
        }

        self.document.draw(&self.canvas);
    }
}
