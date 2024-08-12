use clap::{ArgAction, Parser, Subcommand};
use config::{Config, ConfigError};
use geoprox_core::models::GeoShardConfig;
use geoprox_server::config::ServerConfig;
use serde::Deserialize;
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Specify a config file
    #[arg(short, long, value_name = "CONFIG", env = "GEOPROX_CONFIG")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start Geoprox server
    Run {
        /// The address the server will bind to
        #[arg(short, long, default_value = "0.0.0.0", env = "GEOPROX_HTTP_ADDR")]
        addr: IpAddr,

        /// The port the server will listen on
        #[arg(short, long, default_value_t = 5000, env = "GEOPROX_HTTP_PORT")]
        port: u16,
    },

    /// Hash latitude/longitude into geohash
    Encode {
        /// Latitude
        #[arg(long)]
        lat: f64,

        /// Longitude
        #[arg(long)]
        lng: f64,

        /// Geohash length
        #[arg(short, long, default_value_t = geoprox_core::shard::GeoShard::DEFAULT_DEPTH)]
        depth: usize,
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
pub struct GeoproxConfig {
    pub server: Option<ServerConfig>,
    pub shard: Option<GeoShardConfig>,
}

pub fn runtime() -> Result<(Option<Commands>, GeoproxConfig), ConfigError> {
    let cli = Cli::parse();

    let settings: GeoproxConfig = match cli.config.as_deref() {
        Some(config_path) => Config::builder()
            .add_source(config::File::from(config_path))
            .build()?
            .try_deserialize()?,
        None => Config::builder()
            .add_source(config::File::with_name("/etc/geoprox/geoprox"))
            .build()
            .unwrap_or_default()
            .try_deserialize()
            .unwrap_or_default(),
    };

    Ok((cli.command, settings))
}
