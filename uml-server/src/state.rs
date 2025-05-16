use actix_web::rt::{self};
use actix_ws::{AggregatedMessage, Session};
use futures_util::{StreamExt, stream::FuturesUnordered};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
};
use uml_common::document::Document;

use crate::client_handler::{ClientHandler, WsMessage};

pub enum StateEvent {
    ClientConnected(ClientHandler),
    ClientDisconnected,
    ClientReceived(WsMessage),
    StopSignal,
}

async fn read_message(handlers: &mut [ClientHandler]) -> StateEvent {
    if handlers.is_empty() {
        return futures::future::pending::<StateEvent>().await;
    }

    let mut readers = FuturesUnordered::new();

    for handler in handlers.iter_mut() {
        let fut = handler.read();
        readers.push(fut);
    }

    let Some(msg) = readers.next().await else {
        unreachable!("guarded by if-statement");
    };

    match msg {
        Some(msg) => StateEvent::ClientReceived(msg),
        None => StateEvent::ClientDisconnected,
    }
}

async fn wait_for_event(
    handlers: &mut [ClientHandler],
    new_clients_rx: &mut Receiver<ClientHandler>,
    stop_signal_rx: &mut Receiver<()>,
) -> StateEvent {
    tokio::select! {
        read_event = read_message(handlers) => {
            read_event
        },
        client_handler = new_clients_rx.recv() => {
            let Some(handler) = client_handler else {
                return StateEvent::StopSignal;
            };

            StateEvent::ClientConnected(handler)
        },
        _ = stop_signal_rx.recv() => {
            StateEvent::StopSignal
        }
    }
}

async fn handle_event(
    latest_document: &mut Document,
    handlers: &mut Vec<ClientHandler>,
    event: StateEvent,
) {
    match event {
        StateEvent::ClientConnected(mut client_handler) => {
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
        StateEvent::ClientDisconnected => {
            handlers.retain(|handler| {
                let closed = handler.is_closed();

                if closed {
                    log::debug!(
                        "Removing ClientHandler with ID {} as it has closed.",
                        handler.id()
                    );
                }

                !closed
            });
        }
        StateEvent::ClientReceived(msg) => {
            log::debug!("Client with ID {} received a message.", msg.recipient);
            *latest_document = msg.document;

            for handler in handlers {
                if handler.id() != msg.recipient {
                    let _ = handler.send(msg.json.clone()).await;
                }
            }
        }
        StateEvent::StopSignal => (),
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

                if matches!(event, StateEvent::StopSignal) {
                    log::debug!(
                        "Stop signal received, stopping client handlers."
                    );

                    let mut futures = FuturesUnordered::new();

                    for handler in handlers {
                        futures.push(handler.close());
                    }

                    while (futures.next().await).is_some() {}
                    break;
                }

                handle_event(&mut latest_document, &mut handlers, event).await;
            }
        });

        Self {
            new_clients_tx,
            stop_signal_tx,
            task: Some(task),
        }
    }
}
