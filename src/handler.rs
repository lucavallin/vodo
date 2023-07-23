use log::info;
use rand::Rng;
use std::net::{Ipv4Addr, UdpSocket};

use crate::{
    packet::DnsPacket,
    pb::{BufferError, PacketBuffer},
    question::{DnsQuestion, QueryType},
    rc::ResultCode,
};

// IP of *a.root-servers.net*
const A_ROOT_SERVERS_IP: Ipv4Addr = Ipv4Addr::new(198, 41, 0, 4);
// UDP socket port for lookups
const LOOKUP_SOCKET_PORT: u16 = 42069;

// This function takes a UDP socket as input.
// It receives a DNS query from the socket, and sends a response back.
// If an error occurs, it returns the error.
pub fn handle_query(socket: &UdpSocket) -> Result<(), BufferError> {
    let mut req_buffer = PacketBuffer::new();
    let (_, src) = socket.recv_from(&mut req_buffer.buf)?;

    let mut request = DnsPacket::from_buffer(&mut req_buffer)?;

    let mut packet = DnsPacket::new();
    packet.header.id = request.header.id;
    packet.header.recursion_desired = true;
    packet.header.recursion_available = true;
    packet.header.response = true;

    if let Some(question) = request.questions.pop() {
        info!("Received query: {:?}", question);

        if let Ok(result) = recursive_lookup(&question.name, question.qtype) {
            packet.questions.push(question.clone());
            packet.header.rescode = result.header.rescode;

            for rec in result.answers {
                info!("Answer: {:?}", rec);
                packet.answers.push(rec);
            }
            for rec in result.authorities {
                info!("Authority: {:?}", rec);
                packet.authorities.push(rec);
            }
            for rec in result.resources {
                info!("Resource: {:?}", rec);
                packet.resources.push(rec);
            }
        } else {
            packet.header.rescode = ResultCode::SERVFAIL;
        }
    } else {
        packet.header.rescode = ResultCode::FORMERR;
    }

    let mut res_buffer = PacketBuffer::new();
    packet.write(&mut res_buffer)?;

    let len = res_buffer.pos();
    let data = res_buffer.get_range(0, len)?;

    socket.send_to(data, src)?;

    Ok(())
}

// This function takes a domain name, a query type, and a server address as input.
// It creates a UDP socket, and sends a DNS query to the server.
// It then waits for a response from the server, and returns the response.
// If an error occurs, it returns the error.
fn lookup(
    qname: &str,
    qtype: QueryType,
    server: (Ipv4Addr, u16),
) -> Result<DnsPacket, BufferError> {
    // Socket into which the response is received.
    let socket = UdpSocket::bind(("0.0.0.0", LOOKUP_SOCKET_PORT))?;

    let mut packet = DnsPacket::new();

    packet.header.id = rand::thread_rng().gen();
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = PacketBuffer::new();
    packet.write(&mut req_buffer)?;
    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server)?;

    let mut res_buffer = PacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf)?;

    DnsPacket::from_buffer(&mut res_buffer)
}

// This function takes a domain name and a query type as input.
// It starts by looking up the name in the root servers, and then follows the chain of
// referrals until it finds the authoritative name server for the domain.
// It then looks up the domain name in the authoritative name server, and returns the
// result.
// If an error occurs, it returns the error.
fn recursive_lookup(qname: &str, qtype: QueryType) -> Result<DnsPacket, BufferError> {
    // For now we're always starting with *a.root-servers.net*.
    let mut ns = A_ROOT_SERVERS_IP;

    // It might take an arbitrary number of steps, therefore it uses an unbounded loop.
    loop {
        info!("attempting lookup of {:?} {} with ns {}", qtype, qname, ns);

        // The next step is to send the query to the active server.
        let ns_copy = ns;

        let server = (ns_copy, 53);
        let response = lookup(qname, qtype, server)?;

        // If there are entries in the answer section, and no errors, it's done
        if !response.answers.is_empty() && response.header.rescode == ResultCode::NOERROR {
            return Ok(response);
        }

        // `NXDOMAIN` is a possible reply, which is the authoritative name servers
        // way of telling us that the name doesn't exist.
        if response.header.rescode == ResultCode::NXDOMAIN {
            return Ok(response);
        }

        // Otherwise, try to find a new nameserver based on NS and a corresponding A
        // record in the additional section. If this succeeds, switch name server
        // and retry the loop.
        if let Some(new_ns) = response.get_resolved_ns(qname) {
            ns = new_ns;

            continue;
        }

        // If not, it must resolve the ip of a NS record. If no NS records exist,
        // it uses what the last server said.
        let new_ns_name = match response.get_unresolved_ns(qname) {
            Some(x) => x,
            None => return Ok(response),
        };

        // Starting a new lookup sequence in the midst of our current one.
        //  Hopefully, this will return the IP of an appropriate name server.
        let recursive_response = recursive_lookup(new_ns_name, QueryType::A)?;

        // Finally, pick a random ip from the result, and restart the loop. If no such
        // record is available, it returns the last result received.
        if let Some(new_ns) = recursive_response.get_random_a() {
            ns = new_ns;
        } else {
            return Ok(response);
        }
    }
}
