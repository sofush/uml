use event::Event;
use gloo::{events::EventListener, utils::window};
use html_canvas::HtmlCanvas;
use log::Level;
use mouse_button::MouseButton;
use state::{SHARED_STATE, State};
use uml_common::document::Document;
use wasm_bindgen::prelude::*;

mod camera;
mod cursor;
mod event;
mod html_canvas;
mod mouse_button;
mod state;

fn on_resize(callback: impl Fn(Event) -> () + 'static) {
    EventListener::new(&window(), "resize", move |_| {
        callback(Event::Resize);
    })
    .forget();
}

fn on_mouse_move(callback: impl Fn(Event) -> () + 'static) {
    EventListener::new(&window(), "mousemove", move |e| {
        let event = e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        let x = event.client_x() as u32;
        let y = event.client_y() as u32;
        callback(Event::MouseMove { x, y });
    })
    .forget();
}

fn on_mouse_down(callback: impl Fn(Event) -> () + 'static) {
    EventListener::new(&window(), "mousedown", move |e| {
        let event = e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        let x = event.client_x() as u32;
        let y = event.client_y() as u32;
        let Ok(button) = MouseButton::try_from(event.button()) else {
            return;
        };
        let event = Event::MouseDown { button, x, y };
        callback(event);
    })
    .forget();
}

fn on_mouse_up(callback: impl Fn(Event) -> () + 'static) {
    EventListener::new(&window(), "mouseup", move |e| {
        let event = e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        let x = event.client_x() as u32;
        let y = event.client_y() as u32;
        let Ok(button) = MouseButton::try_from(event.button()) else {
            return;
        };
        let event = Event::MouseUp { x, y, button };
        callback(event);
    })
    .forget();
}

fn on_key_down(callback: impl Fn(Event) -> () + 'static) {
    EventListener::new(&window(), "keydown", move |e| {
        let event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let event = Event::KeyDown { key: event.key() };
        callback(event);
    })
    .forget();
}

fn on_key_up(callback: impl Fn(Event) -> () + 'static) {
    EventListener::new(&window(), "keyup", move |e| {
        let event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let event = Event::KeyUp { key: event.key() };
        callback(event);
    })
    .forget();
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Debug).unwrap();

    let canvas = HtmlCanvas::new();
    canvas.update_size();

    let document = Document::default();
    SHARED_STATE.set(Some(State::new(document, canvas)));

    let event_handler = move |event: Event| {
        SHARED_STATE.with_borrow_mut(|state| {
            let Some(state) = state else {
                panic!("State must always have a value.");
            };

            state.handle_event(event);
        })
    };

    on_resize(event_handler);
    on_mouse_down(event_handler);
    on_mouse_up(event_handler);
    on_mouse_move(event_handler);
    on_key_down(event_handler);
    on_key_up(event_handler);
    Ok(())
}
