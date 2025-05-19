use crate::{
    event::{
        Event, Outcome,
        cursor_style::CursorStyle,
        handler::{
            DragHandler, HoverHandler, KeypressHandler, WebsocketHandler,
        },
    },
    html_canvas::HtmlCanvas,
    wsclient::WsClient,
};
use gloo::{net::websocket::Message, utils::document};
use std::{cell::RefCell, thread_local};
use uml_common::{
    camera::Camera, document::Document, drawable::Drawable,
    interaction::Interactive,
};
use wasm_bindgen::JsCast as _;

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
    document: Document,
    ws: Option<WsClient>,

    canvas: HtmlCanvas,
    camera: Camera,

    drag_handler: DragHandler,
    websocket_handler: WebsocketHandler,
    keypress_handler: KeypressHandler,
    hover_handler: HoverHandler,
}

impl State {
    pub fn new(canvas: HtmlCanvas) -> Self {
        Self {
            document: Document::default(),
            ws: None,

            canvas,
            camera: Camera::default(),

            drag_handler: DragHandler::default(),
            websocket_handler: WebsocketHandler::default(),
            keypress_handler: KeypressHandler::default(),
            hover_handler: HoverHandler::default(),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        log::trace!("Handling event: {event}...");

        if let Event::Redraw = event {
            self.document.draw(&self.canvas, &self.camera);
            return;
        }

        if let Event::Resize = event {
            self.canvas.update_size();
            self.document.draw(&self.canvas, &self.camera);
            return;
        }

        if let Event::Initialize = event {
            let ws = match WsClient::new() {
                Ok(ws) => ws,
                Err(e) => {
                    log::error!("Could not connect WebSocket: {e}");
                    todo!("notify the user and attempt reconnect");
                }
            };

            self.ws = Some(ws);
            self.document.draw(&self.canvas, &self.camera);
            return;
        }

        let mut outcomes = vec![];

        outcomes.extend_from_slice(&self.drag_handler.handle(
            &event,
            self.document.elements_mut(),
            self.camera,
        ));
        outcomes.push(self.websocket_handler.handle(&event));
        outcomes.push(self.keypress_handler.handle(&event, &self.camera));
        outcomes.extend_from_slice(&self.hover_handler.handle(
            &event,
            self.document.elements_mut(),
            &self.camera,
        ));

        let mut sync = false;

        for outcome in &outcomes {
            self.handle_outcome(outcome.clone());
            sync |= matches!(
                outcome,
                Outcome::AddElement(_) | Outcome::MoveElement { .. }
            );
        }

        if sync {
            self.sync_document();
        }
    }

    fn handle_outcome(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::None => (),
            Outcome::Translate { x, y } => {
                self.camera.translate(x as _, y as _);
            }
            Outcome::MoveElement { id, x, y } => {
                if let Some(el) = self
                    .document
                    .elements_mut()
                    .iter_mut()
                    .find(|e| e.id() == id)
                {
                    el.adjust_position(x, y);
                }
            }
            Outcome::ClickElement { id, x, y } => {
                if let Some(el) = self
                    .document
                    .elements_mut()
                    .iter_mut()
                    .find(|e| e.id() == id)
                {
                    el.click(x - el.x(), y - el.y());
                }
            }
            Outcome::CursorStyle(style) => self.set_cursor(style),
            Outcome::UpdateInfo { visible } => {
                self.update_info_element(visible)
            }
            Outcome::UpdateDocument(mut document) => {
                for el in document.elements_mut() {
                    el.initalize(&self.canvas);
                }

                self.document = document;
            }
            Outcome::AddElement(mut element) => {
                element.initalize(&self.canvas);
                self.document.elements_mut().push(element);
            }
            Outcome::HoverElement { id, hovered } => {
                let Some(el) = self
                    .document
                    .elements_mut()
                    .iter_mut()
                    .find(|el| el.id() == id)
                else {
                    return;
                };

                el.get_interaction_mut().set_hover(hovered);
            }
        }
    }

    pub fn sync_document(&mut self) {
        log::trace!("Synchronizing document with server.");

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
    }

    pub fn set_cursor(&self, cursor: CursorStyle) {
        let d = document();
        let Some(canvas) = d.get_element_by_id("canvas") else {
            log::error!("Could not find canvas HTML element.");
            return;
        };

        let Some(el) = canvas.dyn_ref::<web_sys::HtmlCanvasElement>() else {
            log::error!("Could not get cast canvas element into HTML element.");
            return;
        };

        let value = match cursor {
            CursorStyle::Default => "default",
            CursorStyle::Grab => "grab",
            CursorStyle::Grabbing => "grabbing",
        };

        if el.style().set_property("cursor", value).is_err() {
            log::error!("Could not set cursor style.");
        }
    }

    pub fn update_info_element(&mut self, visible: impl Into<Option<bool>>) {
        let text = format!("{}x {}y", self.camera.x(), self.camera.y());
        self.document.update_info(visible.into(), text);
    }
}
