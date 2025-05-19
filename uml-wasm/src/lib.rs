use crate::event::Event;
use crate::mouse_button::MouseButton;
use crate::state::{SHARED_STATE, State};

use event::{KeyboardEvent, MouseEvent};
use gloo::{
    events::{EventListener, EventListenerOptions},
    utils::{document, window},
};
use html_canvas::HtmlCanvas;
use log::Level;
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
        callback(MouseEvent::Move { x, y }.into());
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
        let event = MouseEvent::Down { button, x, y };
        callback(event.into());
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
        let event = MouseEvent::Up { x, y, button };
        callback(event.into());
    })
}

fn on_mouse_out(callback: impl Fn(Event) + 'static) {
    add_event_listener("mouseout", move |e| {
        let event = e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        let x = event.client_x();
        let y = event.client_y();
        callback(MouseEvent::Out { x, y }.into());
    })
}

fn on_mouse_enter(callback: impl Fn(Event) + 'static) {
    add_event_listener("mouseenter", move |e| {
        let event = e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        let x = event.client_x();
        let y = event.client_y();
        callback(MouseEvent::Enter { x, y }.into());
    })
}

fn on_key_down(callback: impl Fn(Event) + 'static) {
    add_event_listener("keydown", move |e| {
        let event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let event = KeyboardEvent::Down { key: event.key() };
        callback(event.into());
    })
}

fn on_key_up(callback: impl Fn(Event) + 'static) {
    add_event_listener("keyup", move |e| {
        let event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let event = KeyboardEvent::Up { key: event.key() };
        callback(event.into());
    })
}

fn on_contextmenu() {
    let Some(canvas) = document().get_element_by_id("canvas") else {
        return;
    };

    let cb = |e: &web_sys::Event| {
        e.prevent_default();
    };

    EventListener::new_with_options(
        &canvas,
        "contextmenu",
        EventListenerOptions::enable_prevent_default(),
        cb,
    )
    .forget()
}

#[wasm_bindgen]
pub fn on_redraw() {
    state::handle_event(Event::Redraw);
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Debug).unwrap();
    std::panic::set_hook(Box::new(|info| log::error!("{info}")));

    let canvas = HtmlCanvas::new();
    canvas.update_size();

    SHARED_STATE.set(Some(State::new(canvas)));
    state::handle_event(Event::Initialize);

    on_resize(state::handle_event);
    on_mouse_down(state::handle_event);
    on_mouse_up(state::handle_event);
    on_mouse_move(state::handle_event);
    on_mouse_out(state::handle_event);
    on_mouse_enter(state::handle_event);
    on_key_down(state::handle_event);
    on_key_up(state::handle_event);
    on_contextmenu();

    Ok(())
}
