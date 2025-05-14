use std::sync::Arc;

use actix_web::rt::{self, task};
use actix_ws::{AggregatedMessage, Session};
use futures_util::{
    StreamExt,
    lock::{Mutex, MutexGuard},
    stream::FuturesUnordered,
};
use tokio::sync::mpsc::Receiver;
use uml_common::document::Document;

use crate::{client_handler::ClientHandler, id::Id};

async fn wait_for_message(
    handlers: &mut MutexGuard<'_, Vec<ClientHandler>>,
) -> Option<(Id, String, Document)> {
    let mut futures = FuturesUnordered::new();

    for handler in handlers.iter_mut() {
        let fut = handler.read();
        futures.push(fut);
    }

    futures.next().await.flatten()
}

#[derive(Default)]
pub struct State {
    handlers: Arc<Mutex<Vec<ClientHandler>>>,
    document: Arc<Mutex<Document>>,
    task: Option<task::JoinHandle<()>>,
}

impl State {
    pub async fn add_connection(
        &mut self,
        session: Session,
        rx: Receiver<AggregatedMessage>,
    ) {
        let handlers = Arc::clone(&self.handlers);
        let document = Arc::clone(&self.document);

        self.task.as_ref().map(|t| t.abort());
        self.task = Some(rt::spawn(async move {
            let mut handlers = handlers.lock().await;
            let mut document = document.lock().await;
            let mut client = ClientHandler::new(session, rx);
            let id = client.id();

            let json = serde_json::to_string(&*document);

            if let Ok(json) = json {
                if client.send(json).await.is_ok() {
                    handlers.push(client);
                    log::debug!("Added ClientHandler with ID {}", id);
                } else {
                    log::warn!(
                        "Attempted to add client with ID {}, but connection is closed.",
                        id
                    );
                };
            } else {
                log::error!(
                    "New client with ID {} was not sent the document as it couldn't be deserialized.",
                    id
                );
            }

            loop {
                handlers.retain(|handler| {
                    let closed = handler.is_closed();

                    if closed {
                        log::debug!("Removing ClientHandler with ID {} as it has closed.", handler.id());
                    }

                    !closed
                });

                let Some((sender_id, json, doc)) =
                    wait_for_message(&mut handlers).await
                else {
                    log::debug!("A client has disconnected.");
                    continue;
                };

                *document = doc;

                for handler in handlers.iter_mut() {
                    if handler.id() != sender_id {
                        let _ = handler.send(json.clone()).await;
                    }
                }
            }
        }));
    }

    pub async fn close_connections(&mut self) {
        self.task = None;
        let mut clients = self.handlers.lock().await;

        for client in clients.drain(..) {
            client.close().await;
        }
    }
}
