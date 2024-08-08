use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub const DEFAULT_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
pub const DEFAULT_PORT: u16 = 5000;
pub const DEFAULT_COUNT: usize = 100;
pub const DEFAULT_SORTED: bool = false;

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub http_addr: Option<std::net::IpAddr>,
    pub http_port: Option<u16>,
    pub default_count: Option<usize>,
    pub default_sorted: Option<bool>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            http_addr: Some(DEFAULT_ADDR),
            http_port: Some(DEFAULT_PORT),
            default_count: Some(DEFAULT_COUNT),
            default_sorted: Some(DEFAULT_SORTED),
        }
    }
}

impl From<ServerConfig> for SocketAddr {
    fn from(config: ServerConfig) -> Self {
        SocketAddr::new(
            config.http_addr.unwrap_or(DEFAULT_ADDR),
            config.http_port.unwrap_or(DEFAULT_PORT),
        )
    }
}
