use clap::{Parser, Subcommand};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use geoprox_server;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// run web server
    RunServer {
        /// <addr>:<port> (default 127.0.0.1:5000)
        #[arg(short, long)]
        bind: Option<SocketAddr>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::RunServer { bind }) => {
            geoprox_server::run(bind.unwrap_or(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                5000,
            )));
        }
        None => {}
    }
}
