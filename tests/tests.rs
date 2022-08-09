extern crate reliable_udp;
use reliable_udp::packet::PType;

#[test]
fn verify_checksum() {
    let header_checksum = reliable_udp::packet::Header::calculate_header_checksum(0, 0, PType::Syn);

    assert_eq!(header_checksum, 1u16);

    let data: [u8; 5] = [1, 2, 3, 4, 5];

    let checksum = reliable_udp::packet::Header::calculate_checksum(
        0,
        0,
        PType::Syn,
        header_checksum,
        Some(&data),
    );

    assert_eq!(checksum, 2312u16);
}
