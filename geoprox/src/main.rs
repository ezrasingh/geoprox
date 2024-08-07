use geoprox_core::models::GeoShardConfig;
use geoprox_server::config::ServerConfig;

mod cli;

fn main() {
    let (command, settings) = cli::runtime().unwrap();

    match &command {
        Some(cli::Commands::Run { bind }) => {
            let server_conf: ServerConfig = settings.server.unwrap_or_default();
            let shard_conf: GeoShardConfig = settings.shard.unwrap_or_default();

            if let Some(socket) = bind {
                // ? merge arguments with any existing config
                geoprox_server::run(
                    ServerConfig {
                        http_addr: Some(socket.ip()),
                        http_port: Some(socket.port()),
                    },
                    shard_conf,
                )
            } else {
                geoprox_server::run(server_conf, shard_conf);
            }
        }

        Some(cli::Commands::Encode { lat, lng, depth }) => {
            let ghash = geoprox_core::geohash::encode(
                [*lng, *lat].into(),
                depth.unwrap_or(geoprox_core::cache::SpatialIndex::DEFAULT_DEPTH),
            )
            .unwrap();
            println!("{}", ghash);
        }

        Some(cli::Commands::Decode { ghash }) => {
            let (position, lng_err, lat_err) = geoprox_core::geohash::decode(ghash).unwrap();
            println!("latitude: {} +/- {}", position.y, lat_err);
            println!("longitude:{} +/- {}", position.x, lng_err);
        }
        None => {
            println!("Invalid command. Please try '--help' for more information.");
        }
    }
}
