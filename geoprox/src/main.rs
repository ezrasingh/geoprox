use clap::{Parser, Subcommand};
use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, path::PathBuf};

use geoprox_server;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// specify a config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// run web server
    Run {
        /// <addr>:<port> (default 127.0.0.1:5000)
        #[arg(short, long)]
        bind: Option<SocketAddr>,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(Commands::Run { bind }) => {
            geoprox_server::run(bind.unwrap_or(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                5000,
            )));
        }
        None => {}
    }
}
