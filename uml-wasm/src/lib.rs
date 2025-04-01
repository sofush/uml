use std::sync::{Arc, Mutex};

use gloo::{events::EventListener, utils::window};
use html_canvas::HtmlCanvas;
use log::Level;
use uml_common::{
    color::BLACK, document::Document, drawable::Drawable, elements::Rectangle,
};
use wasm_bindgen::prelude::*;
use web_sys::Event;

mod html_canvas;

fn on_resize(event_handler: Arc<Mutex<impl FnMut(&Event) -> () + 'static>>) {
    EventListener::new(&window(), "resize", move |e| {
        let mut callback = event_handler.lock().unwrap();
        callback(e);
    })
    .forget();
}

fn on_click(event_handler: Arc<Mutex<impl FnMut(&Event) -> () + 'static>>) {
    EventListener::new(&window(), "click", move |e| {
        let mut callback = event_handler.lock().unwrap();
        callback(e);
    })
    .forget();
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Debug).unwrap();

    let canvas = HtmlCanvas::new();
    let mut document = Document::default();

    canvas.update_size();

    let event_handler = Arc::new(Mutex::new(move |event: &Event| {
        match event.type_().as_str() {
            "resize" => canvas.update_size(),
            "click" => {
                document
                    .elements()
                    .push(Rectangle::new(0, 0, 50, 50, BLACK).into());
            }
            t => log::error!("Unhandled event of type {t}"),
        };

        document.draw(&canvas);
    }));

    on_resize(Arc::clone(&event_handler));
    on_click(Arc::clone(&event_handler));
    Ok(())
}
