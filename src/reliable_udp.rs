use crate::errors::*;

pub enum PType{
    Syn = 1,
    SynAck,
    Ack,
    Fin
}

pub struct Packet{
    pub seq:u32,
    pub ack:u32,

    pub ptype:PType
}


impl Packet {
    pub fn parse(data:&[u8]) -> Result<Packet>{
        if data.len() < 65{
            return Err(packet_parsing_errors::TooSmallPacket.into());
        }

        let seq:u32 = u32::from_ne_bytes(data[0..4].try_into().unwrap());

        let ack:u32 = u32::from_ne_bytes(data[4..8].try_into().unwrap());

        let ptype:u8 = data[8];
        match ptype{
            1 => {Ok(Packet{seq:seq,ack:ack,ptype:PType::Syn})},
            2 => {Ok(Packet{seq:seq,ack:ack,ptype:PType::SynAck})},
            3 => {Ok(Packet{seq:seq,ack:ack,ptype:PType::Ack})},
            4 => {Ok(Packet{seq:seq,ack:ack,ptype:PType::Fin})},
            _ => {Err(packet_parsing_errors::UknownPType::new(ptype).into())}
        }

        
    }
}