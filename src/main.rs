mod handler;
mod header;
mod packet;
mod pb;
mod question;
mod rc;
mod record;

use handler::handle_query;
use log::warn;
use std::{error::Error, net::UdpSocket};

fn main() -> Result<(), Box<dyn Error>> {
    // Bind an UDP socket on port 2053
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;

    // For now, queries are handled sequentially, so an infinite loop for servicing
    // requests is initiated.
    loop {
        match handle_query(&socket) {
            Ok(_) => {}
            Err(e) => warn!("An error occurred: {}", e),
        }
    }
}
