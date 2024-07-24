use clap::{Parser, Subcommand};
use config::Config;
use geoprox_core::shard::GeoShardConfig;
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

use geoprox_core;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// specify a config file
    #[arg(short, long, value_name = "CONFIG")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// start geoprox server
    Run {
        /// <addr>:<port> (default 127.0.0.1:5000)
        #[arg(short, long)]
        bind: Option<SocketAddr>,
    },
    
    /// hash latitude/longitude into geohash
    Encode {
        /// latitude
        #[arg(long)]
        lat: f64,
        
        /// longitude
        #[arg(long)]
        lng: f64,
        
        /// geohash length (default 6)
        #[arg(short, long)]
        depth: Option<usize>,
    },

    /// decode geohash into approximate longitude/latitude
    Decode {
        /// geohash
        #[arg(short, long)]
        ghash: String,
        
    }
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig{
    pub http_addr: Option<IpAddr>,
    pub http_port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct GeoProxConfig{
    pub server: Option<ServerConfig>,
    pub shard: Option<GeoShardConfig>,
}

pub fn runtime() -> (Option<Commands>, GeoProxConfig) {
    let cli = Cli::parse();
    let settings: GeoProxConfig = match cli.config.as_deref() {
        Some(config_path) => Config::builder()
            .add_source(config::File::from(config_path))
            .add_source(config::Environment::with_prefix("GEOPROX"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap(),
        None => Config::builder()
            .add_source(config::File::with_name("geoprox/geoprox.toml"))
            .add_source(config::Environment::with_prefix("GEOPROX"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap(),
    };
    (cli.command, settings)
}
