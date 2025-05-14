use event::Event;
use gloo::{events::EventListener, utils::window};
use html_canvas::HtmlCanvas;
use log::Level;
use mouse_button::MouseButton;
use state::{SHARED_STATE, State};
use wasm_bindgen::prelude::*;

mod event;
mod html_canvas;
mod mouse_button;
mod state;
mod wsclient;

fn add_event_listener(
    event: &'static str,
    callback: impl FnMut(&web_sys::Event) + 'static,
) {
    EventListener::new(&window(), event, callback).forget();
}

fn on_resize(callback: impl Fn(Event) + 'static) {
    add_event_listener("resize", move |_| {
        callback(Event::Resize);
    })
}

fn on_mouse_move(callback: impl Fn(Event) + 'static) {
    add_event_listener("mousemove", move |e| {
        let event = e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        let x = event.client_x();
        let y = event.client_y();
        callback(Event::MouseMove { x, y });
    })
}

fn on_mouse_down(callback: impl Fn(Event) + 'static) {
    add_event_listener("mousedown", move |e| {
        let event = e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        let x = event.client_x();
        let y = event.client_y();
        let Ok(button) = MouseButton::try_from(event.button()) else {
            return;
        };
        let event = Event::MouseDown { button, x, y };
        callback(event);
    })
}

fn on_mouse_up(callback: impl Fn(Event) + 'static) {
    add_event_listener("mouseup", move |e| {
        let event = e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        let x = event.client_x();
        let y = event.client_y();
        let Ok(button) = MouseButton::try_from(event.button()) else {
            return;
        };
        let event = Event::MouseUp { x, y, button };
        callback(event);
    })
}

fn on_key_down(callback: impl Fn(Event) + 'static) {
    add_event_listener("keydown", move |e| {
        let event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let event = Event::KeyDown { key: event.key() };
        callback(event);
    })
}

fn on_key_up(callback: impl Fn(Event) + 'static) {
    add_event_listener("keyup", move |e| {
        let event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let event = Event::KeyUp { key: event.key() };
        callback(event);
    })
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Debug).unwrap();

    let canvas = HtmlCanvas::new();
    canvas.update_size();

    SHARED_STATE.set(Some(State::new(canvas)));

    state::handle_event(Event::Initialize);

    on_resize(state::handle_event);
    on_mouse_down(state::handle_event);
    on_mouse_up(state::handle_event);
    on_mouse_move(state::handle_event);
    on_key_down(state::handle_event);
    on_key_up(state::handle_event);
    Ok(())
}
