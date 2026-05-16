use std::net::SocketAddr;

struct Tcp {
    listen_ip: SocketAddr,
    proxies: Vec<String>,
    forward_to: SocketAddr
}