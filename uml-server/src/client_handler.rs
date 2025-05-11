#[derive(Clone)]
pub struct ClientHandler {
    session: actix_ws::Session,
}

impl ClientHandler {
    pub fn new(session: actix_ws::Session) -> Self {
        Self { session }
    }

    pub async fn handle(
        &mut self,
        message: actix_ws::AggregatedMessage,
    ) -> Result<(), actix_ws::Closed> {
        use actix_ws::AggregatedMessage::*;

        log::debug!("Received WebSocket message: {message:?}");

        match message {
            Ping(bytes) => self.session.pong(&bytes).await,
            _ => Ok(()),
        }
    }
}
