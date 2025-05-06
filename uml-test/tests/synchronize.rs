use std::sync::Arc;
use std::time::Duration;

use futures::future::{self, join_all};
use rand::{Rng, thread_rng as rng};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::sleep;

const SOCKET_ADDRESS: &str = "127.0.0.1:9999";
const NUM_CLIENTS: usize = 100;
const NUM_ITERATIONS: usize = 30;

// #[derive(Debug, Clone, Copy, RandGen)]
// pub enum Change {
//     Add(i32),
//     Subtract(i32),
// }

// #[derive(Debug, Clone, Copy)]
// pub struct Synced(i32);
//
// impl Synced {
//     pub fn increment(&mut self) {
//         self.0 += 1;
//     }
//     // pub fn apply(&mut self, change: Change) {
//     //     match change {
//     //         Add(num) => self.0 += num,
//     //         Subtract(num) => self.0 -= num,
//     //     }
//     // }
// }

async fn run_client() -> anyhow::Result<i32> {
    let stream = TcpStream::connect(SOCKET_ADDRESS).await?;
    let (reader, writer) = stream.into_split();

    let synced = Arc::new(Mutex::new(0));
    let tasks = vec![
        tokio::spawn(send_changes(writer, Arc::clone(&synced))),
        tokio::spawn(read_changes(reader, Arc::clone(&synced))),
    ];

    join_all(tasks).await;
    Ok(*synced.lock().await)
}

async fn send_changes(mut writer: OwnedWriteHalf, synced: Arc<Mutex<i32>>) {
    let mut write = async move || {
        let n = *synced.lock().await;
        writer.write_i32(n + 1).await
    };

    while write().await.is_ok() {
        let ms = rng().gen_range(15..35);
        sleep(Duration::from_millis(ms)).await;
    }
}

async fn read_changes(mut reader: OwnedReadHalf, synced: Arc<Mutex<i32>>) {
    while let Ok(n) = reader.read_i32().await {
        *synced.lock().await = n;
    }
}

async fn handle_client(
    mut reader: OwnedReadHalf,
    client_pool: Arc<Mutex<Vec<OwnedWriteHalf>>>,
    synced: Arc<Mutex<i32>>,
) {
    for _ in 0..NUM_ITERATIONS {
        let Ok(new_value) = reader.read_i32().await else {
            break;
        };

        *synced.lock().await = new_value;

        for client in client_pool.lock().await.iter_mut() {
            let _ = client.write_i32(new_value).await;
            let _ = client.flush().await;
        }
    }
}

async fn run_server() -> io::Result<i32> {
    let listener = TcpListener::bind(SOCKET_ADDRESS).await?;
    let writers = Arc::new(Mutex::new(vec![]));
    let synced = Arc::new(Mutex::new(0));
    let mut tasks = vec![];

    for _ in 0..NUM_CLIENTS {
        let (stream, _) = listener.accept().await?;
        let (reader, writer) = stream.into_split();

        writers.lock().await.push(writer);
        tasks.push(handle_client(
            reader,
            Arc::clone(&writers),
            Arc::clone(&synced),
        ));
    }

    join_all(tasks).await;
    Ok(*synced.lock().await)
}

#[tokio::test]
async fn synchronize() -> anyhow::Result<()> {
    let client_handles = (0..NUM_CLIENTS)
        .map(|_| tokio::spawn(async move { run_client().await }))
        .collect::<Vec<_>>();

    let num = run_server().await?;
    eprintln!("Serverens resultat: {num}");

    future::join_all(client_handles)
        .await
        .into_iter()
        .map(|result| result.expect("all futures should be joinable"))
        .map(|result| result.expect("no client should return an error"))
        .for_each(|ret| assert_eq!(ret, num));

    Ok(())
}
