use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub mod packet_parsing_errors{
    use crate::errors::{*};

    #[derive(Debug, Clone)]
    pub struct TooSmallPacket;
    impl fmt::Display for TooSmallPacket {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "Packet should be at least 65 bytes")
        }
    }
    impl error::Error for TooSmallPacket {}

    #[derive(Debug, Clone)]
    pub struct UknownPType{
        pub ptype:u8
    }
    impl UknownPType{
        pub fn new(ptype:u8) -> UknownPType{
            UknownPType { ptype:ptype }
        }
    }
    impl fmt::Display for UknownPType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "Unknown packet type: {}", self.ptype)
        }
    }
    impl error::Error for UknownPType {}
}