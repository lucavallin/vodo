mod handler;
mod header;
mod packet;
mod pb;
mod question;
mod rc;
mod record;

#[macro_use]
extern crate log;
extern crate simplelog;

use clap::Parser;
use handler::handle_query;
use simplelog::*;
use std::{error::Error, net::UdpSocket};

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    /// Port for the server to listen on
    #[arg(short, long = "port", default_value_t = 5353)]
    port: u16,
}

/// Entry point of the server.
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging.
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();

    // Parse command line arguments.
    let args = Args::parse();

    // Bind an UDP socket the specified port.
    let socket = UdpSocket::bind(("0.0.0.0", args.port))?;

    // Queries are handled sequentially, so an infinite loop for servicing requests is initiated.
    info!("DNS server is listening on port {}...", args.port);
    loop {
        match handle_query(&socket) {
            Ok(_) => {}
            Err(e) => warn!("An error occurred: {}", e),
        }
    }
}
