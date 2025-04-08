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
    task: Option<task::JoinHandle<()>>,
}

impl State {
    pub async fn add_connection(
        &mut self,
        session: Session,
        rx: Receiver<AggregatedMessage>,
    ) {
        let handlers = Arc::clone(&self.handlers);

        self.task.as_ref().map(|t| t.abort());
        self.task = Some(rt::spawn(async move {
            let handlers = &mut handlers.lock().await;
            handlers.push(ClientHandler::new(session, rx));

            loop {
                handlers.retain(|h| !h.is_closed());

                let Some((sender_id, json, _doc)) =
                    wait_for_message(handlers).await
                else {
                    log::warn!("wait_for_message returned None");
                    continue;
                };

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

// let handler = Arc::new(Mutex::new(ClientHandler::new(session)));
// let handler_clone = handler.clone();
// let clients = self.clients.clone();
//
// let task_handle = rt::spawn(async move {
//     while let Some(msg) = stream.recv().await {
//         let res = match msg {
//             Ok(msg) => {
//                 let mut handler = handler.lock().await;
//                 handler.handle(msg).await
//             }
//             Err(e) => {
//                 log::error!(
//                     "ClientHandler failed to handle message: {e}"
//                 );
//                 break;
//             }
//         };
//
//         let Some((json, document)) = res else {
//             log::debug!(
//                 "WebSocket received message could not be deserialized to a valid document."
//             );
//             continue;
//         };
//
//         let clients = clients.lock().await;
//
//         for client in clients.iter() {
//             if Arc::ptr_eq(&client.handler, &handler) {
//                 continue;
//             }
//
//             let mut handler = client.handler.lock().await;
//             handler.send(json.clone()).await;
//         }
//     }
// });
//
// let mut clients = self.clients.lock().await;
//
// clients.push(Client {
//     handler: handler_clone,
//     task_handle,
// });
