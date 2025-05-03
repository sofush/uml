use std::time::Duration;

use crate::diagram::Diagram;
use tokio::{
    io::{AsyncReadExt as _, AsyncWriteExt},
    net::TcpStream,
    select,
};

pub struct Client {
    diagram: Diagram,
    stream: TcpStream,
}

impl Client {
    pub async fn new(addr: &str) -> anyhow::Result<Self> {
        let stream = TcpStream::connect(addr).await?;

        Ok(Self {
            diagram: Diagram::default(),
            stream,
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let mut timer = tokio::time::interval(Duration::from_millis(1000));

        select! {
            res = self.stream.read_u32() => {
                let mut buf = vec![0u8; res? as usize];
                let size = self.stream.read(&mut buf).await?;
                buf.truncate(size);
                let message = String::from_utf8(buf)?;
                println!("message: {message:?}");
                self.diagram = serde_json::from_str(&message)?;
                println!("Recevied: {:?}", self.diagram);
            }
            _ = timer.tick() => {
                println!("tick");
                let json = serde_json::to_string(&self.diagram)?;
                let bytes = json.as_bytes();
                self.stream.write_u32(bytes.len() as u32).await?;
                self.stream.write_all(bytes).await?;
            }
        };

        Ok(())
    }

    pub fn into_inner(self) -> Diagram {
        self.diagram
    }
}
