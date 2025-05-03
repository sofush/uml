use std::sync::Arc;
use std::time::Duration;

use anyhow::bail;
use futures::future::{self, join_all};
use rand::{Rng, thread_rng};
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use uml_test::diagram::Diagram;

const SOCKET_ADDRESS: &str = "127.0.0.1:9999";
const NUM_CLIENTS: usize = 100;

pub struct DiagramWriter {
    inner: Box<&'static mut (dyn AsyncWrite + Unpin)>,
}

pub struct DiagramReader {
    inner: Box<&'static mut (dyn AsyncRead + Unpin)>,
    cached_diagram: Diagram,
}

impl DiagramWriter {
    pub async fn write(&mut self, diagram: Diagram) -> anyhow::Result<()> {
        let json = serde_json::to_string(&diagram)?;
        self.inner.write_u32(json.len() as _).await?;
        self.inner.write(json.as_bytes()).await?;
        Ok(())
    }
}

impl DiagramReader {
    pub async fn read(&mut self) -> anyhow::Result<Diagram> {
        let size = self.inner.read_u32().await? as usize;
        let mut buf = vec![0u8; size];
        let read = self.inner.read(&mut buf).await?;

        if size != read {
            bail!("Could not read diagram.");
        }

        let diagram: Diagram = serde_json::from_slice(&buf)?;
        self.cached_diagram = diagram.clone();
        Ok(diagram)
    }
}

// pub async fn write_diagram(
//     writer: &mut (impl AsyncWriteExt + Unpin),
//     diagram: &Diagram,
// ) -> io::Result<()> {
//     let json: String =
//         serde_json::to_string(diagram).expect("diagram is always serializable");
//
//     writer.write_u32(json.len() as u32).await?;
//     writer.write_all(json.as_bytes()).await
// }
//
// pub async fn read_diagram<W: AsyncReadExt + Unpin>(
//     reader: &mut W,
// ) -> io::Result<Diagram> {
//     let size = reader.read_u32().await?;
//     let mut buf = vec![0u8; size as usize];
//     reader.read_exact(&mut buf).await?;
//
//     let json =
//         String::from_utf8(buf).expect("messages should always be vaild UTF-8");
//     let diagram =
//         serde_json::from_str(&json).expect("diagram is always serializable");
//
//     Ok(diagram)
// }

pub async fn run_client(idx: usize) -> io::Result<Diagram> {
    let stream = TcpStream::connect(SOCKET_ADDRESS).await?;
    let (mut reader, mut writer) = stream.into_split();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Diagram>(1);
    println!("Client {idx} connected to the server.");

    let handle: JoinHandle<tokio::io::Result<()>> = tokio::spawn(async move {
        loop {
            let diagram = read_diagram(&mut reader).await?;
            tx.send(diagram).await.unwrap();
        }
    });

    let mut cached_diagram = Diagram::default();
    let dur = Duration::from_millis(thread_rng().gen_range(500..800));
    let mut interval = tokio::time::interval(dur);

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let new_diagram = apply_random_change(&cached_diagram);

                println!("Client {idx} is sending its changes.");

                if write_diagram(&mut writer, &new_diagram).await.is_err() {
                    continue;
                };
            }
            value = rx.recv() => {
                println!("Client {idx} received a new value.");

                cached_diagram = match value {
                    Some(v) => v,
                    None => break,
                }
            }
        }
    }

    let _ = handle.await;

    println!("Client {idx} is closing...");
    Ok(cached_diagram)
}

pub async fn run_server() -> io::Result<Diagram> {
    let listener = TcpListener::bind(SOCKET_ADDRESS).await?;
    let mut readers = vec![];
    let mut writers = vec![];

    for _ in 0..NUM_CLIENTS {
        let (client, addr) = listener.accept().await?;
        let (reader, writer) = client.into_split();
        println!("Client with port {} has connected.", addr.port());
        readers.push(reader);
        writers.push(writer);
    }

    let writers = Arc::new(Mutex::new(writers));
    let cached_diagram = Arc::new(Mutex::new(Diagram::default()));
    let mut reader_futures = vec![];

    for mut reader in readers.into_iter() {
        let writers = writers.clone();
        let cached_diagram = cached_diagram.clone();

        let fut = tokio::spawn(async move {
            let Ok(diagram) = read_diagram(&mut reader).await else {
                return;
            };

            *cached_diagram.lock().await = diagram.clone();

            for writer in writers.lock().await.iter_mut() {
                write_diagram(writer, &diagram).await.unwrap();
                writer.flush().await.unwrap();
            }
        });

        reader_futures.push(fut);
    }

    join_all(reader_futures).await;

    println!("Server is closing...");
    Ok(cached_diagram.lock().await.clone())
}

#[tokio::test]
async fn synchronize() -> anyhow::Result<()> {
    let server_handle = tokio::spawn(async { run_server().await });
    let client_handles = (0..NUM_CLIENTS)
        .map(|idx| tokio::spawn(async move { run_client(idx).await }))
        .collect::<Vec<_>>();

    let diagram = server_handle
        .await
        .expect("server should not return an error")?;

    future::join_all(client_handles)
        .await
        .into_iter()
        .map(|result| result.expect("all futures should be joinable"))
        .map(|result| result.expect("no client should return an error"))
        .for_each(|ret| assert_eq!(ret, diagram));

    Ok(())
}
