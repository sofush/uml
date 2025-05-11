use std::{rc::Rc, sync::Arc};

use futures::{SinkExt as _, StreamExt as _, lock::Mutex, stream::SplitSink};
use gloo::{
    net::websocket::{Message, WebSocketError, futures::WebSocket},
    utils::errors::JsError,
};
use wasm_bindgen_futures::spawn_local;

use crate::{event::Event, state};

#[derive(Debug, Clone)]
pub enum WsEvent {
    Received(Message),
    ReceiveError(Rc<WebSocketError>),
    SendError(Rc<WebSocketError>),
}

pub struct WsClient {
    writer: Arc<Mutex<SplitSink<WebSocket, Message>>>,
}

impl WsClient {
    pub fn new() -> Result<Self, JsError> {
        log::debug!("Connecting WebSocket...");
        let ws = WebSocket::open("/websocket")?;
        log::debug!("WebSocket connected!");

        let (writer, mut reader) = ws.split();

        spawn_local(async move {
            while let Some(msg) = reader.next().await {
                let wsevent = match msg {
                    Ok(msg) => WsEvent::Received(msg),
                    Err(e) => WsEvent::ReceiveError(Rc::new(e)),
                };

                state::handle_event(Event::WebSocket(wsevent));
            }
        });

        Ok(Self {
            writer: Arc::new(Mutex::new(writer)),
        })
    }

    pub fn send(&mut self, items: Vec<Message>) -> Result<(), WebSocketError> {
        let writer = self.writer.clone();

        spawn_local(async move {
            let mut writer = writer.lock().await;

            for item in items {
                if let Err(e) = writer.feed(item.to_owned()).await {
                    log::error!("Could not feed WebSocket: {e}");
                    let event =
                        Event::WebSocket(WsEvent::SendError(Rc::new(e)));
                    state::handle_event(event);
                    return;
                };
            }

            if let Err(e) = writer.flush().await {
                log::error!("Could not flush WebSocket: {e}");
                let event = Event::WebSocket(WsEvent::SendError(Rc::new(e)));
                state::handle_event(event);
            };
        });

        Ok(())
    }
}
