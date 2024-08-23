use duration_string::DurationString;
use serde::Deserialize;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    time::Duration,
};

pub const DEFAULT_CONFIG_PATH: &str = "/etc/geoprox";

#[derive(Clone, Debug, Deserialize)]
pub struct SnapshotConfig {
    /// Directory where snapshots will be stored and read from (default /etc/geoprox/)
    pub path: Option<PathBuf>,
    /// Determines how often snapshots will be taken (default 60s)
    pub every: Option<DurationString>,
}

impl SnapshotConfig {
    pub const SNAPSHOT_FILENAME: &'static str = "snapshot.bin";
    pub fn bin_path(&self) -> PathBuf {
        self.path
            .clone()
            .unwrap_or(PathBuf::from(DEFAULT_CONFIG_PATH))
            .join(Self::SNAPSHOT_FILENAME)
    }
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            path: Some(PathBuf::from(DEFAULT_CONFIG_PATH)),
            every: Some(DurationString::default()),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    /// The address the server will bind to (default 0.0.0.0)
    pub http_addr: Option<IpAddr>,
    /// The port the server will listen on (default 5000)
    pub http_port: Option<u16>,
    /// Timeout duration in seconds (default 10s)
    pub timeout: Option<DurationString>,
    /// Determines how snapshots will be handled
    pub snapshot: SnapshotConfig,
}

impl ServerConfig {
    pub const DEFAULT_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
    pub const DEFAULT_PORT: u16 = 5000;
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            http_addr: Some(Self::DEFAULT_ADDR),
            http_port: Some(Self::DEFAULT_PORT),
            timeout: Some(DurationString::new(Duration::from_secs(10))),
            snapshot: SnapshotConfig::default(),
        }
    }
}

impl From<ServerConfig> for SocketAddr {
    fn from(config: ServerConfig) -> Self {
        SocketAddr::new(
            config.http_addr.unwrap_or(ServerConfig::DEFAULT_ADDR),
            config.http_port.unwrap_or(ServerConfig::DEFAULT_PORT),
        )
    }
}
