use log::info;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream, UdpSocket, tcp::{OwnedReadHalf, OwnedWriteHalf}}, sync::Mutex, time::Instant};
use crate::proxy::ProxyConfig;

pub struct Udp {
    pub listen_ip: SocketAddr,
    pub proxies: Vec<ProxyConfig>,
    pub forward_to: SocketAddr,
}

#[derive(Clone)]
struct Session {
    last_packet: Instant,
    socket: UdpSocket
}

impl Udp {
    pub async fn start_loop(&self) {
        let socket = UdpSocket::bind(self.listen_ip)
            .await
            .expect("Failed to start UDP listener");
        if self.listen_ip.port() == 0 {
            info!(
                "Listening UDP on {}",
                socket
                    .local_addr()
                    .expect("Failed to get UDP listener local address")
            );
        };
        let mut buffer = vec![0; 65535];
        let mut sessions: Arc<Mutex<HashMap<SocketAddr, Session>>> = Arc::new(Mutex::new(HashMap::new()));
        loop {
            let Ok((size, peer)) = socket.recv_from(&mut buffer).await else { continue };
            info!("Got packet with len {}", size);
            let option = { 
                let sessions = sessions.lock().await; 
                sessions.get(&peer).cloned()
            };
            match option {
                Some(session) => {
                    session.socket.send_to(&mut buffer[..size], self.forward_to).await.ok();
                },
                None => {
                    
                    //sessions.insert(peer, Session { last_packet: Instant::now(), socket: socket })
                }
            };
        }
    }
}
