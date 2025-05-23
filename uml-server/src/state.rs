use actix_web::rt::{self};
use actix_ws::{AggregatedMessage, Session};
use futures::future::select_all;
use futures_util::{StreamExt, stream::FuturesUnordered};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
};
use uml_common::document::Document;

use crate::client_handler::{ClientHandler, WsMessage};

enum Event {
    ClientConnected(ClientHandler),
    ClientReceived(WsMessage),
    StopSignal,
}

async fn read_message(handlers: &mut [ClientHandler]) -> Event {
    if handlers.is_empty() {
        return futures::future::pending::<Event>().await;
    }

    let messages = handlers.iter_mut().map(|h| Box::pin(h.read()));
    let (message, ..) = select_all(messages).await;
    Event::ClientReceived(message)
}

async fn wait_for_event(
    handlers: &mut [ClientHandler],
    new_clients_rx: &mut Receiver<ClientHandler>,
    stop_signal_rx: &mut Receiver<()>,
) -> Event {
    tokio::select! {
        message = read_message(handlers) => {
            message
        },
        client_handler = new_clients_rx.recv() => {
            let Some(handler) = client_handler else {
                return Event::StopSignal;
            };

            Event::ClientConnected(handler)
        },
        _ = stop_signal_rx.recv() => {
            Event::StopSignal
        }
    }
}

async fn handle_event(
    latest_document: &mut Document,
    handlers: &mut Vec<ClientHandler>,
    event: Event,
) {
    match event {
        Event::ClientConnected(mut client_handler) => {
            let Ok(json) = serde_json::to_string(&latest_document) else {
                log::warn!(
                    "Client with ID {} was not sent document, as it could not be deserialized.",
                    client_handler.id(),
                );
                return;
            };

            let Ok(()) = client_handler.send(json).await else {
                log::warn!(
                    "Attempted to add client with ID {}, but connection is closed.",
                    client_handler.id(),
                );
                return;
            };

            log::debug!("Client with ID {} connected!", client_handler.id());
            handlers.push(client_handler);
        }
        Event::ClientReceived(WsMessage::Document {
            recipient,
            json,
            document,
        }) => {
            log::trace!("Client with ID {} received a message.", recipient);
            *latest_document = document;

            for handler in handlers {
                if handler.id() != recipient {
                    let _ = handler.send(json.clone()).await;
                }
            }
        }
        Event::ClientReceived(WsMessage::DeserializeError {
            recipient,
            error: _,
        }) => {
            log::debug!(
                "Could not deserialize a message received from client with ID {}.",
                recipient
            );
            if let Some(index) =
                handlers.iter().position(|c| c.id() == recipient)
            {
                handlers.remove(index);
                log::debug!("Client with ID {} has been removed.", recipient);
            }
        }
        Event::ClientReceived(WsMessage::Closed { recipient }) => {
            log::debug!(
                "Removing client with ID {} because it has disconnected.",
                recipient
            );

            if let Some(index) =
                handlers.iter().position(|c| c.id() == recipient)
            {
                handlers.remove(index);
                log::debug!("Client with ID {} has been removed.", recipient);
            }
        }
        Event::StopSignal => {
            log::debug!("Stop signal received, closing connection to clients.");

            let mut futures = FuturesUnordered::new();
            let handlers = handlers.drain(..);

            for handler in handlers {
                futures.push(handler.close());
            }

            while (futures.next().await).is_some() {}
        }
    }
}

pub struct State {
    new_clients_tx: Sender<ClientHandler>,
    stop_signal_tx: Sender<()>,
    task: Option<JoinHandle<()>>,
}

impl State {
    pub async fn add_connection(
        &mut self,
        session: Session,
        rx: Receiver<AggregatedMessage>,
    ) {
        let client = ClientHandler::new(session, rx);
        self.new_clients_tx
            .send(client)
            .await
            .expect("receive half should always be open");
    }

    pub async fn stop(&mut self) {
        let Some(task) = self.task.take() else {
            return;
        };

        let _ = self.stop_signal_tx.send(()).await;
        let _ = task.await;
    }
}

impl Default for State {
    fn default() -> Self {
        let (stop_signal_tx, mut stop_signal_rx) =
            tokio::sync::mpsc::channel::<()>(1);

        let (new_clients_tx, mut new_clients_rx) =
            tokio::sync::mpsc::channel::<ClientHandler>(100);

        let task = rt::spawn(async move {
            let mut handlers: Vec<ClientHandler> = vec![];
            let mut latest_document = Document::default();

            loop {
                let event = wait_for_event(
                    &mut handlers,
                    &mut new_clients_rx,
                    &mut stop_signal_rx,
                )
                .await;

                let stop = matches!(event, Event::StopSignal);
                handle_event(&mut latest_document, &mut handlers, event).await;

                if stop {
                    break;
                }
            }
        });

        Self {
            new_clients_tx,
            stop_signal_tx,
            task: Some(task),
        }
    }
}
