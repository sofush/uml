use actix_ws::AggregatedMessage::{self, *};
use tokio::sync::mpsc::Receiver;
use uml_common::document::Document;

use crate::id::Id;

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

    pub async fn read(&mut self) -> Option<(Id, String, Document)> {
        let msg = self.stream.recv().await?;
        log::trace!("Received WebSocket message: {msg:?}");

        let Text(text) = msg else {
            return None;
        };

        let json = text.to_string();
        let document = serde_json::from_str(&json).ok()?;
        Some((self.id, json, document))
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
        let _ = self.session.close(Some(close_reason)).await;
    }

    pub fn is_closed(&self) -> bool {
        self.stream.is_closed()
    }
}

impl PartialEq for ClientHandler {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ClientHandler {}
