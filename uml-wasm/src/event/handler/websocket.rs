use gloo::net::websocket::Message;

use crate::{
    event::{Event, Outcome},
    wsclient::WsEvent,
};

#[derive(Debug, Default)]
pub struct WebsocketHandler {}

impl WebsocketHandler {
    pub fn handle(&mut self, event: &Event) -> Outcome {
        let Event::WebSocket(ev) = event else {
            return Outcome::None;
        };

        match ev {
            WsEvent::Received(msg) => {
                return self.handle_message(msg);
            }
            WsEvent::ReceiveError(err) => {
                log::error!(
                    "WebSocket client failed to receive messages from server: {err}"
                );
            }
            WsEvent::SendError(err) => {
                log::error!("WebSocket client failed to send a message: {err}");
            }
        }

        Outcome::None
    }

    fn handle_message(&mut self, msg: &Message) -> Outcome {
        log::trace!("WebSocket message: {msg:?}");

        let Message::Text(str) = msg else {
            log::error!("Bytes message received from server (not supported).");
            return Outcome::None;
        };

        let Ok(document) = serde_json::from_str(str) else {
            log::error!("Could not deserialize text message from server.");
            return Outcome::None;
        };

        Outcome::UpdateDocument(document)
    }
}
