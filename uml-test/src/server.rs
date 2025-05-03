use std::sync::Arc;

use futures::lock::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpListener, TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    task::JoinHandle,
};

pub enum ServerEvent {
    ClientConnected(TcpStream),
    MessageReceived(String),
}

pub struct Server {
    accept_task: JoinHandle<tokio::io::Result<()>>,
    writers: Arc<Mutex<Vec<OwnedWriteHalf>>>,
}

async fn handle_reader(
    mut reader: OwnedReadHalf,
    writers: Arc<Mutex<Vec<OwnedWriteHalf>>>,
) -> anyhow::Result<()> {
    let size = reader.read_u32().await?;
    let mut buf = vec![0u8; size as usize];
    let size = reader.read(&mut buf).await?;
    buf.truncate(size);

    {
        let message = String::from_utf8(buf.clone()).unwrap();
        println!("Received: {}", message);
    }

    for writer in writers.lock().await.iter_mut() {
        writer.write_u32(size as _).await?;
        writer.write_all(&buf).await?;
    }

    Ok(())
}

impl Server {
    pub async fn new(addr: &str) -> anyhow::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        let writers = Default::default();

        Ok(Server {
            writers: Arc::clone(&writers),
            accept_task: tokio::spawn(async move {
                loop {
                    let writers = Arc::clone(&writers);
                    let (stream, _) = listener.accept().await?;
                    let (reader, writer) = stream.into_split();
                    writers.lock().await.push(writer);

                    tokio::spawn(async move {
                        let _ =
                            handle_reader(reader, Arc::clone(&writers)).await;
                    });
                }
            }),
        })
    }

    pub async fn broadcast(&mut self, message: String) -> anyhow::Result<()> {
        let mut writers = self.writers.lock().await;

        for writer in writers.iter_mut() {
            writer.write_all(message.as_bytes()).await?;
        }

        Ok(())
    }

    pub async fn run(self) -> anyhow::Result<()> {
        self.accept_task.await??;
        Ok(())
    }
}
