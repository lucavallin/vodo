mod handler;
mod header;
mod packet;
mod pb;
mod question;
mod rc;
mod record;

use clap::Parser;
use handler::handle_query;
use log::warn;
use std::{error::Error, net::UdpSocket};

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    /// Port for the server to listen on
    #[arg(short, long = "port", default_value_t = 2053)]
    port: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Bind an UDP socket the specified port.
    let socket = UdpSocket::bind(("0.0.0.0", args.port))?;

    // Queries are handled sequentially, so an infinite loop for servicing requests is initiated.
    loop {
        match handle_query(&socket) {
            Ok(_) => {}
            Err(e) => warn!("An error occurred: {}", e),
        }
    }
}
