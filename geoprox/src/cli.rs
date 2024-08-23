use clap::{ArgAction, Parser, Subcommand};
use geoprox_core::models::GeoShardConfig;
use geoprox_server::config::ServerConfig;
use serde::Deserialize;
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start Geoprox server
    Run {
        /// The address the server will bind to
        #[arg(short, long, env = "GEOPROX_HTTP_ADDR")]
        addr: Option<IpAddr>,

        /// The port the server will listen on
        #[arg(short, long, env = "GEOPROX_HTTP_PORT")]
        port: Option<u16>,

        /// Specify a config file
        #[arg(short, long, env = "GEOPROX_CONFIG")]
        config_path: Option<PathBuf>,
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
        #[arg(short, long, default_value = "openapi.json")]
        filename: String,

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
