use std::time::Duration;

use actix_ws::AggregatedMessage::{self, *};
use tokio::sync::mpsc::Receiver;
use uml_common::document::Document;

use crate::id::Id;

pub enum WsMessage {
    Document {
        recipient: Id,
        json: String,
        document: Document,
    },
    Closed {
        recipient: Id,
    },
    DeserializeError {
        recipient: Id,
        #[allow(unused)]
        error: serde_json::Error,
    },
}

pub struct ClientHandler {
    id: Id,
    session: actix_ws::Session,
    stream: Receiver<AggregatedMessage>,
}

impl ClientHandler {
    pub fn new(
        session: actix_ws::Session,
        stream: Receiver<AggregatedMessage>,
    ) -> Self {
        Self {
            session,
            stream,
            id: Id::default(),
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub async fn read(&mut self) -> WsMessage {
        let msg = self.stream.recv().await;
        log::trace!(
            "Client with ID {} received a WebSocket message: {msg:?}",
            self.id
        );

        let json = match msg {
            Some(Text(text)) => text.to_string(),
            _ => {
                return WsMessage::Closed {
                    recipient: self.id(),
                };
            }
        };

        match serde_json::from_str(&json) {
            Ok(document) => WsMessage::Document {
                recipient: self.id(),
                json,
                document,
            },
            Err(e) => WsMessage::DeserializeError {
                recipient: self.id(),
                error: e,
            },
        }
    }

    pub async fn send(
        &mut self,
        document_json: String,
    ) -> Result<(), actix_ws::Closed> {
        self.session.text(document_json).await
    }

    pub async fn close(self) {
        let close_reason = actix_ws::CloseReason {
            code: actix_ws::CloseCode::Restart,
            description: None,
        };

        let _ = tokio::time::timeout(Duration::from_millis(1000), async {
            let _ = self.session.close(Some(close_reason)).await;
        })
        .await;
    }
}

impl PartialEq for ClientHandler {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ClientHandler {}
