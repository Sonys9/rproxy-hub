use log::info;
use std::net::SocketAddr;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}}};
use crate::proxy::ProxyConfig;

pub struct Tcp {
    pub listen_ip: SocketAddr,
    pub proxies: Vec<ProxyConfig>,
    pub forward_to: SocketAddr,
}

async fn process_data(mut writer: OwnedWriteHalf, mut reader: OwnedReadHalf) -> Result<(), std::io::Error> {
    let mut buffer = [0; 1024];
    loop {
        let n = reader.read(&mut buffer).await?;
        if n == 0 {
            return Ok::<(), std::io::Error>(())
        };
        info!("Got packet with len {}", n);
        writer.write_all(&buffer[..n]).await?;
    }
}

async fn handle_client(stream: TcpStream, forward_to: SocketAddr, proxy: ProxyConfig) -> Result<(), std::io::Error> {
    let connection = TcpStream::connect(forward_to).await?;
    connection.set_nodelay(true).ok();
    let (client_reader, client_writer) = stream.into_split();
    let (server_reader, server_writer) = connection.into_split();
    tokio::select! {
        res = tokio::spawn(process_data(server_writer, client_reader)) => res?,
        res = tokio::spawn(process_data(client_writer, server_reader)) => res?
    }
}

impl Tcp {
    pub async fn start_loop(&self) {
        let listener = TcpListener::bind(self.listen_ip)
            .await
            .expect("Failed to start TCP listener");
        if self.listen_ip.port() == 0 {
            info!(
                "Listening tcp on {}",
                listener
                    .local_addr()
                    .expect("Failed to get tcp listener local address")
            );
        };
        let mut count = 0;
        loop {
            let Ok((stream, addr)) = listener.accept().await else {
                continue;
            };
            stream.set_nodelay(true).ok();
            let proxy = self.proxies[count % self.proxies.len()].clone();
            info!("New tcp client: {}. Using proxy {:?}", addr, proxy);
            tokio::spawn(handle_client(stream, self.forward_to, proxy));
            count += 1;
        }
    }
}
