extern crate reliable_udp;
// extern crate tokio;
use rand::Rng;

use reliable_udp::manager;
use reliable_udp::packet;
use std::error::Error;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    let socket = UdpSocket::bind("0.0.0.0:5050").await?;

    let mut buffer: [u8; 1024] = [0; 1024];

    let (_, addr) = socket.recv_from(&mut buffer).await?;
    let packet_header = reliable_udp::packet::Header::parse(&buffer[..packet::HEADER_SIZE])?;
    //let data = &buffer[packet::HEADER_SIZE..size];
    if packet_header.ptype != packet::PType::Syn {
        println!("Not a Syn packet");
        return Ok(());
    }
    if !packet_header.verify_header_checksum() || !packet_header.verify_checksum(None) {
        println!("Bad checksum");
        return Ok(());
    }

    let seq = rng.gen();
    let mut connection = manager::Connection {
        seq: seq,
        ack: packet_header.seq,
        previous_seq: seq,
        is_open: false,
        last_response: 5,
    };

    let header_checksum =
        packet::Header::calculate_header_checksum(seq, connection.ack + 1, packet::PType::SynAck);
    let checksum = packet::Header::calculate_checksum(
        seq,
        connection.ack + 1,
        packet::PType::SynAck,
        header_checksum,
        None,
    );
    let packet_header = packet::Header {
        seq: seq,
        ack: connection.ack + 1,
        ptype: packet::PType::SynAck,
        header_checksum,
        checksum,
    };
    let packet = packet::packet_to_binary(packet_header, None);
    socket.send_to(&packet, addr).await?;

    let (_, addr) = socket.recv_from(&mut buffer).await?;
    let packet_header = reliable_udp::packet::Header::parse(&buffer[..packet::HEADER_SIZE])?;
    if packet_header.ptype != packet::PType::Ack {
        println!("Not a Syn packet");
        return Ok(());
    }
    if !packet_header.verify_header_checksum() || !packet_header.verify_checksum(None) {
        println!("Bad checksum");
        return Ok(());
    }
    if packet_header.ack != connection.seq + 1 || packet_header.seq != connection.ack {
        println!("Packet needs to be resent");
        return Ok(());
    }

    connection.seq += 1;
    connection.is_open = true;

    println!("Connection established");

    // loop{

    // }

    Ok(())
}
