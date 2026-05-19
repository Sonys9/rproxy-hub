use crate::proxy::ProxyConfig;
use log::{debug, info};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpListener, TcpStream, UdpSocket,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::Mutex,
    time::Instant,
};

pub struct Udp {
    pub listen_ip: SocketAddr,
    pub proxies: Vec<ProxyConfig>,
    pub forward_to: SocketAddr,
}

#[derive(Clone)]
struct Session {
    last_packet: Instant,
    socket: Arc<UdpSocket>,
    forward_to: SocketAddr,
    server_socket: Arc<UdpSocket>,
    origin_addr: SocketAddr,
    relay_addr: SocketAddr
}

async fn watch(session: Session) {
    debug!("Waiting for packets from the server...");
    let mut buffer = vec![0; 65535];
    loop {
        let Ok((size, peer)) = session.server_socket.recv_from(&mut buffer).await else {
            continue;
        };
        debug!("Got packet in server socket");
        if peer != session.forward_to {
            continue;
        };
        session
            .socket
            .send_to(&buffer[..size], session.origin_addr)
            .await
            .ok();
    }
}

impl Udp {
    pub async fn start_loop(&self) {
        let socket = Arc::new(
            UdpSocket::bind(self.listen_ip)
                .await
                .expect("Failed to start UDP listener"),
        );
        if self.listen_ip.port() == 0 {
            info!(
                "Listening UDP on {}",
                socket
                    .local_addr()
                    .expect("Failed to get UDP listener local address")
            );
        };
        let mut buffer = vec![0; 65535]; // Max possible buffer (u16 limit)
        let sessions: Arc<Mutex<HashMap<SocketAddr, Session>>> =
            Arc::new(Mutex::new(HashMap::new()));
        loop {
            let Ok((size, peer)) = socket.recv_from(&mut buffer).await else {
                continue;
            };
            info!("Got packet with len {}", size);
            let option = {
                let sessions = sessions.lock().await;
                sessions.get(&peer).cloned()
            };
            match option {
                Some(session) => {
                    session
                        .server_socket
                        .send_to(&buffer[..size], self.forward_to)
                        .await
                        .ok();
                }
                None => {
                    debug!("Got new peer");
                    let mut stream = Socks5Stream::connect(self.proxies[0].addr, self.forward_to).await.expect("Failed to connect to the proxy");
                    let relay_addr = stream.udp_address().expect("Proxy must support UDP");
                    let server_socket = Arc::new(
                        UdpSocket::bind("0.0.0.0:0") // Random available port
                            .await
                            .expect("Failed to start UDP listener"),
                    );
                    server_socket
                        .send_to(&buffer[..size], self.forward_to)
                        .await
                        .ok();
                    {
                        let mut sessions = sessions.lock().await;
                        let session = Session {
                            last_packet: Instant::now(),
                            socket: Arc::clone(&socket),
                            forward_to: self.forward_to,
                            server_socket,
                            origin_addr: peer,
                        };
                        sessions.insert(peer, session.clone());
                        tokio::spawn(watch(session));
                    };
                }
            };
        }
    }
}
