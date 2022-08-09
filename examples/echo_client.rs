extern crate reliable_udp;
// extern crate tokio;
use rand::Rng;

use reliable_udp::manager;
use reliable_udp::packet;
use std::error::Error;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_address = "127.0.0.1:5050";

    let mut rng = rand::thread_rng();

    let socket = UdpSocket::bind("0.0.0.0:4040").await?;

    let mut buffer: [u8; 1024] = [0; 1024];

    let seq = rng.gen();
    let mut connection = manager::Connection {
        seq: seq,
        ack: 0,
        previous_seq: seq,
        is_open: false,
        last_response: 5,
    };

    let header_checksum = packet::Header::calculate_header_checksum(seq, 0, packet::PType::Syn);
    let checksum = packet::Header::calculate_checksum(
        seq,
        connection.ack,
        packet::PType::Syn,
        header_checksum,
        None,
    );
    let packet_header = packet::Header {
        seq: seq,
        ack: connection.ack,
        ptype: packet::PType::Syn,
        header_checksum,
        checksum,
    };
    let packet = packet::packet_to_binary(packet_header, None);
    socket.send_to(&packet, server_address).await?;

    let (_, addr) = socket.recv_from(&mut buffer).await?;
    let packet_header = reliable_udp::packet::Header::parse(&buffer[..packet::HEADER_SIZE])?;
    if packet_header.ptype != packet::PType::SynAck {
        println!("Not a SynAck packet");
        return Ok(());
    }
    if !packet_header.verify_header_checksum() || !packet_header.verify_checksum(None) {
        println!("Bad checksum");
        return Ok(());
    }
    if packet_header.ack != connection.seq + 1 {
        println!("Packet needs to be resent");
        return Ok(());
    }

    connection.ack = packet_header.seq;
    connection.seq += 1;
    connection.is_open = true;

    let header_checksum =
        packet::Header::calculate_header_checksum(seq, connection.ack + 1, packet::PType::Ack);
    let checksum = packet::Header::calculate_checksum(
        seq,
        connection.ack + 1,
        packet::PType::Ack,
        header_checksum,
        None,
    );
    let packet_header = packet::Header {
        seq: seq,
        ack: connection.ack + 1,
        ptype: packet::PType::Ack,
        header_checksum,
        checksum,
    };
    let packet = packet::packet_to_binary(packet_header, None);
    socket.send_to(&packet, server_address).await?;

    connection.ack += 1;

    Ok(())
}
