use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod packet_parsing_errors {
    use super::*;
    use crate::packet::MAX_PACKET_SIZE;

    #[derive(Debug, Clone, Error)]
    #[error("Packet should be at least 65 bytes")]
    pub struct TooSmallPacket;

    #[derive(Debug, Clone, Error)]
    #[error("Unknown packet type: {}", self.ptype)]
    pub struct UknownPType {
        pub ptype: u8,
    }
    impl UknownPType {
        pub fn new(ptype: u8) -> UknownPType {
            UknownPType { ptype }
        }
    }

    #[derive(Debug, Clone, Error)]
    #[error("Too big packet: {}, the max is: {}", self.size, MAX_PACKET_SIZE)]
    pub struct TooBigPacket {
        pub size: usize,
    }
    impl TooBigPacket {
        pub fn new(size: usize) -> TooBigPacket {
            TooBigPacket { size }
        }
    }
}
