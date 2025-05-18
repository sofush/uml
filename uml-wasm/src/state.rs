use crate::{
    drag::DragState,
    event::Event,
    html_canvas::HtmlCanvas,
    mouse_button::MouseButton,
    wsclient::{WsClient, WsEvent},
};
use gloo::{net::websocket::Message, timers::callback::Timeout};
use std::{cell::RefCell, collections::HashSet, thread_local};
use uml_common::{
    camera::Camera,
    document::Document,
    drawable::Drawable,
    elements::{Class, Info, TextProperties},
    id::Id,
    interaction::Interactive,
};

thread_local! {
    pub static SHARED_STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

pub fn handle_event(event: Event) {
    SHARED_STATE.with_borrow_mut(|state| {
        let Some(state) = state else {
            panic!("State must always have a value.");
        };

        state.handle_event(event);
    })
}

pub struct State {
    document: Option<Document>,
    canvas: HtmlCanvas,
    camera: Camera,
    keys_pressed: HashSet<String>,
    cursor_pos: (i32, i32),
    mouse_buttons: HashSet<MouseButton>,
    drag_state: DragState,
    ws: Option<WsClient>,
}

impl State {
    pub fn new(canvas: HtmlCanvas) -> Self {
        Self {
            ws: None,
            document: None,
            canvas,
            camera: Camera::default(),
            keys_pressed: HashSet::new(),
            mouse_buttons: HashSet::new(),
            cursor_pos: (0, 0),
            drag_state: DragState::None,
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        log::trace!("Handling event: {event}...");

        let mut delta_cursor_pos = None;

        match event.clone() {
            Event::MouseDown { button, .. } => {
                self.mouse_buttons.insert(button);
            }
            Event::MouseUp { button, .. } => {
                self.mouse_buttons.remove(&button);
            }
            Event::MouseMove { x, y }
            | Event::MouseEnter { x, y }
            | Event::MouseOut { x, y } => {
                delta_cursor_pos =
                    Some((x - self.cursor_pos.0, y - self.cursor_pos.1));

                self.cursor_pos = (x, y);
                let abs_cursor_pos = self.get_absolute_cursor_pos();

                if let Some(document) = &mut self.document {
                    let visible = !matches!(&event, Event::MouseOut { .. });
                    document.update_cursor(abs_cursor_pos, visible);
                }
            }
            Event::KeyDown { key } => {
                match (&mut self.document, key.as_str(), self.drag_state) {
                    (Some(doc), "a", DragState::None) => {
                        let x = self.cursor_pos.0 + self.camera.x() as i32;
                        let y = self.cursor_pos.1 + self.camera.y() as i32;
                        let class =
                            Class::new(x, y, 100, 100, None, None, Some(3));
                        doc.add_element(class);
                    }
                    _ => (),
                }

                self.keys_pressed.insert(key);
            }
            Event::KeyUp { key } => {
                self.keys_pressed.remove(&key);
            }
            Event::Resize => self.canvas.update_size(),
            Event::Initialize => {
                self.ws = WsClient::new().ok();

                if self.ws.is_none() {
                    static DELAY: u32 = 500;

                    log::debug!(
                        "WebSocket connection failed, retrying in {DELAY}ms..."
                    );

                    Timeout::new(DELAY, move || {
                        self::handle_event(Event::Initialize);
                    })
                    .forget();
                }
            }
            Event::WebSocket(ev) => match ev {
                WsEvent::Received(msg) => {
                    log::trace!("Received WebSocket message: {msg:?}");

                    if let Message::Text(text) = msg {
                        if let Ok(mut document) =
                            serde_json::from_str::<Document>(&text)
                        {
                            document.set_sync(true);
                            self.document = Some(document);
                        }
                    }
                }
                WsEvent::SendError(e) | WsEvent::ReceiveError(e) => {
                    log::error!("Websocket error: {e:?}");
                    self.handle_event(Event::Initialize);
                    return;
                }
            },
            Event::Redraw => {
                if let Some(document) = &self.document {
                    document.draw(&self.canvas, &self.camera);
                }

                if let DragState::Camera = self.drag_state {
                    let props = TextProperties::new(20.0, "Arial");
                    let text =
                        format!("{}x {}y", self.camera.x(), self.camera.y());
                    let info = Info::new(text, props);
                    info.draw_fixed(&self.canvas);
                }
            }
        };

        if event.is_mouse() || event.is_keyboard() {
            let delta_x = delta_cursor_pos.map(|d| d.0).unwrap_or(0);
            let delta_y = delta_cursor_pos.map(|d| d.1).unwrap_or(0);
            self.handle_drag(delta_x, delta_y);
        }

        self.sync_document();
    }

    pub fn sync_document(&mut self) {
        let Some(document) = &mut self.document else {
            return;
        };

        if document.is_sync() {
            return;
        }

        log::debug!("Synchronizing document with server.");

        let Some(ws) = &mut self.ws else {
            log::error!("Could not sync document because ws is None.");
            return;
        };

        let Ok(document_json) = serde_json::to_string(document) else {
            log::error!("Serialization of document failed.");
            return;
        };

        log::trace!("Attempting to synchronize document.");
        ws.send(vec![Message::Text(document_json)]);
        document.set_sync(true);
    }

    pub fn handle_drag(&mut self, delta_x: i32, delta_y: i32) {
        let lmb = self.mouse_buttons.contains(&MouseButton::Left);
        let translate_key = self.keys_pressed.contains(" ");

        let mut move_element = |id: Id| {
            if delta_x == 0 && delta_y == 0 {
                return;
            }

            let Some(doc) = &mut self.document else {
                return;
            };

            let Some(el) =
                doc.elements_mut().iter_mut().find(|el| el.id() == id)
            else {
                return;
            };

            log::trace!(
                "Element with ID {} was moved: ({}, {}).",
                el.id(),
                delta_x,
                delta_y
            );
            el.adjust_position(delta_x, delta_y);
            doc.set_sync(false);
        };

        match self.drag_state {
            DragState::None if lmb => {
                if translate_key {
                    self.drag_state = DragState::Camera;
                    return;
                }

                let Some(doc) = &self.document else {
                    return;
                };

                let abs_cursor_pos = self.get_absolute_cursor_pos();

                if let Some(el) = doc
                    .elements()
                    .iter()
                    .find(|el| el.cursor_intersects(abs_cursor_pos))
                {
                    self.drag_state =
                        DragState::PressingElement { id: el.id() };
                }
            }
            DragState::Camera => {
                self.camera.translate(-delta_x as _, -delta_y as _);
                log::trace!("Camera state after translate: {:?}", self.camera);

                if !lmb || !translate_key {
                    self.drag_state = DragState::None;
                }
            }
            DragState::PressingElement { id } => {
                if lmb {
                    if delta_x == 0 && delta_y == 0 {
                        return;
                    }

                    move_element(id);
                    self.drag_state = DragState::DraggingElement { id };
                    return;
                }

                let (mut x, mut y) = self.get_absolute_cursor_pos();

                let Some(doc) = &mut self.document else {
                    return;
                };

                doc.elements_mut().iter_mut().find(|el| el.id() == id).map(
                    |el| {
                        x -= el.x();
                        y -= el.y();
                        el.click(x, y);
                        log::debug!("Element with ID {} was clicked.", el.id());
                    },
                );

                self.drag_state = DragState::None;
                return;
            }
            DragState::DraggingElement { id } => {
                if !lmb {
                    self.drag_state = DragState::None;
                    self.document.as_mut().map(|d| d.set_sync(false));
                    return;
                }

                move_element(id);
            }
            _ => (),
        }
    }

    pub fn get_absolute_cursor_pos(&self) -> (i32, i32) {
        (
            self.cursor_pos.0 + self.camera.x() as i32,
            self.cursor_pos.1 + self.camera.y() as i32,
        )
    }
}
