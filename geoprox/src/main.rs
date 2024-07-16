use clap::{Parser, Subcommand};
use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, path::PathBuf};

use geoprox_core;
use geoprox_server;
mod config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// specify a config file
    #[arg(short, long, value_name = "CONFIG")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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
        
        /// geohash length (default 10)
        #[arg(short, long)]
        precision: Option<usize>,
    },

    /// decode geohash into approximate longitude/latitude
    Decode {
        /// geohash
        #[arg(short, long)]
        ghash: String,
        
    }
}

fn main() {
    let cli = Cli::parse();
    if let Some(config_path) = cli.config.as_deref() {
        println!("using config: {}", config_path.display());
    }


    match &cli.command {
        Some(Commands::Run { bind }) => {
            let addr: &SocketAddr = match bind {
                Some(socket) => socket,
                None => {
                    &SocketAddr::new(
                        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                        5000,
                    )
                }
            };
            geoprox_server::run(addr);
        },

        Some(Commands::Encode { lat, lng, precision }) => {
            let position : geoprox_core::geohash::Coord<f64> = [*lat, *lng].into();
            let ghash = geoprox_core::geohash::encode(position, precision.unwrap_or(10));
            println!("({}, {}) => {}", lat, lng, ghash.unwrap());
        },

        Some(Commands::Decode { ghash }) => {
            let (position, lng_err, lat_err) = geoprox_core::geohash::decode(ghash).unwrap();
            println!("{:#?}", position);
            println!("latitiude error: +/- {}", lat_err);
            println!("longitude error: +/- {}", lng_err);
        }
        None => {}
    }
}
