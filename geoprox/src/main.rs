use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use geoprox_core;
use geoprox_server;
mod cli;

fn main() {
    let (command, settings) = cli::runtime();

    match &command {
        Some(cli::Commands::Run { bind }) => {
            let addr: &SocketAddr = match bind {
                Some(socket) => socket,
                None => {
                    let default_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
                    let default_port: u16 = 5000;
                    if let Some(server_conf) = settings.server{
                        &SocketAddr::new(
                            server_conf.http_addr.unwrap_or(default_addr), 
                            server_conf.http_port.unwrap_or(default_port)
                        )
                    } else {
                        &SocketAddr::new(
                            default_addr,
                            default_port,
                        )
                    }
                }
            };
            geoprox_server::run(&addr, settings.shard);
        },

        Some(cli::Commands::Encode { lat, lng, depth }) => {
            let ghash = geoprox_core::geohash::encode([*lng, *lat].into(), depth.unwrap_or(geoprox_core::cache::SpatialIndex::DEFAULT_DEPTH)).unwrap();
            println!("({}, {}) => {}", lat, lng, ghash);
        },

        Some(cli::Commands::Decode { ghash }) => {
            let (position, lng_err, lat_err) = geoprox_core::geohash::decode(ghash).unwrap();
            println!("latitude: {} +/- {}", position.y, lat_err);
            println!("longitude:{} +/- {}", position.x, lng_err);
        }
        None => {
            panic!("invalid command")
        }
    }
}
