use std::net::SocketAddr;

use url::Url;

#[derive(Debug, Clone)]
pub enum Protocol {
    Socks5,
    Socks4,
    Http,
    Https,
}

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub proto: Protocol,
    pub addr: SocketAddr,
    pub credentials: Option<(String, String)>,
}

pub fn parse_proxy(line: &str) -> Result<ProxyConfig, String> {
    let parsed_url = Url::parse(line).map_err(|e| format!("Invalid URL structure: {}", e))?;

    let proto = match parsed_url.scheme() {
        "socks5" => Protocol::Socks5,
        "socks4" => Protocol::Socks4,
        "http" => Protocol::Http,
        "https" => Protocol::Https,
        protocol => {
            return Err(format!(
                "Unsupported protocol: {}. Only HTTP, HTTPS, SOCKS v4/v5 are allowed",
                protocol
            ));
        }
    };

    let host = parsed_url
        .host_str()
        .ok_or_else(|| "Missing host/IP".to_string())?;

    let port = parsed_url
        .port()
        .ok_or_else(|| "Missing port".to_string())?;

    let addr_str = format!("{}:{}", host, port);
    let addr: SocketAddr = addr_str
        .parse()
        .map_err(|e| format!("Invalid address {}: {}", addr_str, e))?;

    let credentials = if !parsed_url.username().is_empty() {
        parsed_url
            .password()
            .map(|pass| (parsed_url.username().to_string(), pass.to_string()))
    } else {
        None
    };

    Ok(ProxyConfig {
        proto,
        addr,
        credentials,
    })
}
