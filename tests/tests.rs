extern crate reliable_udp;
use reliable_udp::packet::PType;

#[test]
fn verify_checksum(){
    let packet = reliable_udp::packet::Packet::new(0,0,PType::Syn,5,None).unwrap();

    assert_eq!(packet.checksum,261u16);

    let packet = reliable_udp::packet::Packet::new(20,20,PType::Syn,5,None).unwrap();

    assert_eq!(packet.checksum,301u16);
}
