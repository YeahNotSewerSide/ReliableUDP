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

    pub ptype:PType,
    pub checksum:u16
}


impl Packet {
    pub fn parse(data:&[u8]) -> Result<Packet>{
        if data.len() < 65{
            return Err(packet_parsing_errors::TooSmallPacket.into());
        }

        let seq:u32 = u32::from_be_bytes(data[0..4].try_into()?);

        let ack:u32 = u32::from_be_bytes(data[4..8].try_into()?);

        let ptype:PType = match data[8]{
            1 => {PType::Syn},
            2 => {PType::SynAck},
            3 => {PType::Ack},
            4 => {PType::Fin},
            _ => {return Err(packet_parsing_errors::UknownPType::new(data[8]).into())}
        };

        let checksum:u16 = u16::from_be_bytes(data[8..12].try_into()?); 
        
        Ok(Packet{seq:seq,ack:ack,ptype:ptype,checksum:checksum})

    }
}