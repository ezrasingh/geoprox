use clap::Parser;
use config::Config;
use std::io::Write;

use geoprox_server::config::ServerConfig;

mod cli;
use cli::{Cli, GeoproxConfig};

fn main() {
    let cli = Cli::parse();
    let cwd = std::env::current_dir().unwrap();
    match &cli.command {
        Some(cli::Commands::Run {
            addr,
            port,
            config_path,
        }) => {
            let settings: GeoproxConfig = match config_path.as_deref() {
                Some(config_path) => Config::builder()
                    .add_source(config::File::from(config_path))
                    .build()
                    .unwrap()
                    .try_deserialize()
                    .unwrap(),
                None => GeoproxConfig::default(),
            };
            let server_config = settings.server.unwrap_or_default();
            dbg!("using settings:", &server_config);
            geoprox_server::run(
                ServerConfig {
                    http_addr: addr.or(server_config.http_addr),
                    http_port: port.or(server_config.http_port),
                    timeout: server_config.timeout,
                    snapshots: server_config.snapshots,
                },
                settings.shard.unwrap_or_default(),
            )
        }

        Some(cli::Commands::Encode { lat, lng, depth }) => {
            let ghash = geoprox_core::geohash::encode([*lng, *lat].into(), *depth).unwrap();
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
                    let dir = destination
                        .as_deref() // Convert &Option<PathBuf> to Option<&Path>
                        .unwrap_or(&cwd);

                    dir.join(filename)
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
            eprintln!("Invalid command. Please try '--help' for more information.");
        }
    }
}
