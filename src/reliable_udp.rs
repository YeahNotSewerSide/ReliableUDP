use crate::errors::*;

pub const HEADER_SIZE: usize = 12;
pub const MAX_PACKET_SIZE: usize = 65507;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PType {
    Syn = 1,
    SynAck,
    Ack,
    Fin,
}

pub struct Packet {
    pub seq: u32,
    pub ack: u32,

    pub ptype: PType,
    pub padding: u8,
    pub checksum: u16,

    pub data: Vec<u8>,
}

impl Packet {
    pub fn parse(data: &[u8]) -> Result<Packet> {
        if data.len() < HEADER_SIZE {
            return Err(packet_parsing_errors::TooSmallPacket.into());
        } else if data.len() > MAX_PACKET_SIZE {
            return Err(packet_parsing_errors::TooBigPacket::new(data.len()).into());
        }

        let seq: u32 = u32::from_be_bytes(data[0..4].try_into()?);

        let ack: u32 = u32::from_be_bytes(data[4..8].try_into()?);

        let ptype: PType = match data[8] {
            1 => PType::Syn,
            2 => PType::SynAck,
            3 => PType::Ack,
            4 => PType::Fin,
            _ => return Err(packet_parsing_errors::UknownPType::new(data[8]).into()),
        };

        let padding: u8 = data[9];
        let checksum: u16 = u16::from_be_bytes(data[10..12].try_into()?);

        let payload = data[12..].to_vec();

        Ok(Packet {
            seq,
            ack,
            ptype,
            padding,
            checksum,
            data: payload,
        })
    }

    pub fn calculate_checksum(seq: u32, ack: u32, ptype: PType, padding: u8) -> u16 {
        let mut dump: [u8; HEADER_SIZE - 2] = [0; HEADER_SIZE - 2];

        for (b, i) in seq.to_be_bytes().iter().zip(0..4) {
            dump[i as usize] = *b;
        }

        for (b, i) in ack.to_be_bytes().iter().zip(4..8) {
            dump[i as usize] = *b;
        }

        dump[8] = ptype as u8;

        dump[9] = padding;

        let mut checksum: u16 =
            u16::from_be_bytes(unsafe { dump[0..1].try_into().unwrap_unchecked() });

        for i in (2..10).step_by(2) {
            checksum += u16::from_be_bytes(unsafe { dump[i..i + 1].try_into().unwrap_unchecked() })
        }

        checksum
    }

    pub fn new(seq: u32, ack: u32, ptype: PType, padding: u8, data: Vec<u8>) -> Result<Packet> {
        if data.len() > MAX_PACKET_SIZE - HEADER_SIZE {
            return Err(packet_parsing_errors::TooBigPacket::new(data.len()).into());
        }

        let checksum = Packet::calculate_checksum(seq, ack, ptype, padding);

        Ok(Packet {
            seq,
            ack,
            ptype,
            padding,
            checksum,
            data,
        })
    }

    pub fn verify_checksum(&self) -> bool {
        let calculated_checksum =
            Packet::calculate_checksum(self.seq, self.ack, self.ptype, self.padding);

        self.checksum == calculated_checksum
    }

    pub fn dump(&self) -> Vec<u8> {
        let mut to_return: Vec<u8> = Vec::with_capacity(HEADER_SIZE + self.data.len());

        to_return.extend(self.seq.to_be_bytes());

        to_return.extend(self.ack.to_be_bytes());

        to_return.push(self.ptype as u8);

        to_return.push(self.padding);

        to_return.extend(self.checksum.to_be_bytes());

        to_return
    }
}
