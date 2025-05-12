use crate::{
    event::Event,
    html_canvas::HtmlCanvas,
    mouse_button::MouseButton,
    wsclient::{WsClient, WsEvent},
};
use gloo::{net::websocket::Message, timers::callback::Timeout};
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

pub fn handle_event(event: Event) {
    SHARED_STATE.with_borrow_mut(|state| {
        let Some(state) = state else {
            panic!("State must always have a value.");
        };

        state.handle_event(event);
    })
}

pub struct State {
    document: Document,
    canvas: HtmlCanvas,
    camera: Camera,
    keys_pressed: HashSet<String>,
    cursor_pos: (i32, i32),
    mouse_buttons: HashSet<MouseButton>,
    translate_camera: bool,
    ws: Option<WsClient>,
}

impl State {
    pub fn new(document: Document, canvas: HtmlCanvas) -> Self {
        Self {
            ws: None,
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
                        if let Ok(document) =
                            serde_json::from_str::<Document>(&text)
                        {
                            self.document = document;
                            self.document.assume_sync();
                        }
                    }
                }
                WsEvent::SendError(e) | WsEvent::ReceiveError(e) => {
                    log::error!("Websocket error: {e:?}");
                    self.handle_event(Event::Initialize);
                    return;
                }
            },
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
                let rect = Rectangle::new(x, y, 100, 100, BLACK);
                self.document.add_element(rect);
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

        self.sync_document();
    }

    pub fn sync_document(&mut self) {
        if self.document.synchronized() {
            return;
        }

        log::debug!("Synchronizing document with server.");

        let Some(ws) = &mut self.ws else {
            log::error!("Could not sync document because ws is None.");
            return;
        };

        let Ok(document_json) = serde_json::to_string(&self.document) else {
            log::error!("Serialization of document failed.");
            return;
        };

        log::trace!("Attempting to synchronize document.");
        ws.send(vec![Message::Text(document_json)]);
        self.document.assume_sync();
    }
}
