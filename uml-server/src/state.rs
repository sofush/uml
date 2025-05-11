use actix_web::rt::task;

#[derive(Default)]
pub struct State {
    connections: Vec<task::JoinHandle<()>>,
}

impl State {
    pub fn add_connection(&mut self, connection: task::JoinHandle<()>) {
        self.connections.push(connection);
    }

    pub fn close_connections(&self) {
        for conn in &self.connections {
            conn.abort();
        }
    }
}
