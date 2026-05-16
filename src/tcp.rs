use std::net::SocketAddr;
use log::info;
use tokio::net::{TcpListener, TcpStream};

pub struct Tcp {
    pub listen_ip: SocketAddr,
    pub proxies: Vec<String>,
    pub forward_to: SocketAddr
}

async fn handle_client(stream: TcpStream, forward_to: SocketAddr, proxy: String) {
    loop {
        
    }
}

impl Tcp {
    pub async fn start_loop(&self) {
        let listener = TcpListener::bind(self.listen_ip).await.expect("Failed to start TCP listener");
        if self.listen_ip.port() == 0 {
            info!("Listening tcp on {}", listener.local_addr().expect("Failed to get tcp listener local address"));
        };
        let mut count = 0;
        loop {
            let Ok((stream, addr)) = listener.accept().await else { continue };
            let proxy = self.proxies[count % self.proxies.len()].clone();
            info!("New tcp client: {}. Using proxy {}", addr, proxy);
            tokio::spawn(handle_client(stream, self.forward_to, proxy));
            count += 1;
        }
    }
}