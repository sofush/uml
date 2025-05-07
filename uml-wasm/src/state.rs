use crate::{event::Event, html_canvas::HtmlCanvas};
use std::{cell::RefCell, thread_local};
use uml_common::{
    color::BLACK, document::Document, drawable::Drawable, elements::Rectangle,
};

thread_local! {
    pub static SHARED_STATE: RefCell<Option<State>> = RefCell::new(None);
}

pub struct State {
    document: Document,
    canvas: HtmlCanvas,
}

impl State {
    pub fn new(document: Document, canvas: HtmlCanvas) -> Self {
        Self { document, canvas }
    }

    pub fn handle_event(&mut self, event: Event) {
        log::debug!("Handling event: {event}...");

        match event {
            Event::Click { x, y } => {
                let rect = Rectangle::new(x, y, 10, 10, BLACK);
                self.document.elements().push(rect.into());
            }
            Event::Resize => self.canvas.update_size(),
            Event::MouseMove { x, y } => (),
        };

        self.document.draw(&self.canvas);
    }
}
