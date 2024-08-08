use clap::{ArgAction, Parser, Subcommand};
use config::{Config, ConfigError};
use geoprox_core::models::GeoShardConfig;
use geoprox_server::config::ServerConfig;
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::PathBuf;

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
    /// Start Geoprox server
    Run {
        /// <addr>:<port> (defaults to 0.0.0.0:5000)
        #[arg(short, long)]
        bind: Option<SocketAddr>,
    },

    /// Hash latitude/longitude into geohash
    Encode {
        /// Latitude
        #[arg(long)]
        lat: f64,

        /// Longitude
        #[arg(long)]
        lng: f64,

        /// Geohash length (defaults to 6)
        #[arg(short, long)]
        depth: Option<usize>,
    },

    /// Decode geohash into approximate longitude/latitude
    Decode {
        /// geohash
        #[arg(short, long)]
        ghash: String,
    },

    /// Generate an OpenAPI specification file
    Spec {
        /// Directory path to store the specification file (defaults to the current working directory)
        #[arg(short, long)]
        destination: Option<PathBuf>,

        /// Name of the output file (defaults to openapi.json)
        #[arg(short, long)]
        filename: Option<String>,

        /// Output the API specification to standard output (stdout)
        #[arg(short, long, action=ArgAction::SetTrue)]
        stdout: bool,

        /// Format output JSON
        #[arg(short, long, action=ArgAction::SetTrue)]
        pretty: bool,
    },
}

#[derive(Debug, Default, Deserialize)]
pub struct GeoProxConfig {
    pub server: Option<ServerConfig>,
    pub shard: Option<GeoShardConfig>,
}

pub fn runtime() -> Result<(Option<Commands>, GeoProxConfig), ConfigError> {
    let cli = Cli::parse();
    let settings: GeoProxConfig = match cli.config.as_deref() {
        Some(config_path) => Config::builder()
            .add_source(config::File::from(config_path))
            .add_source(config::Environment::with_prefix("GEOPROX"))
            .build()?
            .try_deserialize()?,
        None => Config::builder()
            .add_source(config::File::with_name("/etc/geoprox/geoprox"))
            .add_source(config::Environment::with_prefix("GEOPROX"))
            .build()
            .unwrap_or_default()
            .try_deserialize()
            .unwrap_or_default(),
    };
    Ok((cli.command, settings))
}
