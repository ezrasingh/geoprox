use std::io::Write;

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
                depth.unwrap_or(geoprox_core::shard::GeoShard::DEFAULT_DEPTH),
            )
            .unwrap();
            println!("{}", ghash);
        }

        Some(cli::Commands::Decode { ghash }) => {
            let (position, lng_err, lat_err) = geoprox_core::geohash::decode(ghash).unwrap();
            println!("Latitude:{} +/- {}", position.y, lat_err);
            println!("Longitude: {} +/- {}", position.x, lng_err);
        }
        Some(cli::Commands::Spec {
            destination,
            filename,
            stdout,
            pretty,
        }) => {
            let spec_json = geoprox_server::api::docs::openapi_spec_json(*pretty).unwrap();
            if *stdout {
                // ? print the spec JSON to stdout, if `stdout` is `true` or `Some(true)`,
                println!("{}", spec_json);
            } else {
                // ? combine the directory and file name into the final path
                let file_path = {
                    let cwd = std::env::current_dir().unwrap();
                    let dir = destination
                        .as_deref() // Convert &Option<PathBuf> to Option<&Path>
                        .unwrap_or(&cwd);

                    let file_name = filename.as_deref().unwrap_or("openapi.json");
                    dir.join(file_name)
                };
                // ? create the file and write the spec JSON to it
                match std::fs::File::create(&file_path) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(spec_json.as_bytes()) {
                            eprintln!("Failed to write to file: {}", e);
                        } else {
                            println!("OpenAPI specification saved to: {:?}", file_path);
                        }
                    }
                    Err(e) => eprintln!("Failed to create file: {}", e),
                }
            }
        }
        None => {
            println!("Invalid command. Please try '--help' for more information.");
        }
    }
}
