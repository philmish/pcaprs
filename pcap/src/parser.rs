use core::fmt;
use byte::{bytes_to_u32,bytes_to_u16, Byte};

use crate::file::LinkType;

pub struct FileHeader {
    bytes: [u8;24],
}

impl FileHeader {
    
    pub fn new(bytes: Vec<u8>) -> Result<Self, &'static str> {
        if bytes.len() < 24 {
            return Err("Insufficent data length to parse header.");
        }
        let mut data: [u8;24] = [0;24];
        let mut c: usize = 0;
        let _: Vec<()> = bytes.iter().map(|x| {
            if c < 24 {
                data[c] = *x
            }
            c += 1;
        }).collect();
        return Ok(Self { bytes: data });
    }

    pub fn is_swapped(&self) -> bool {
        return self.magic_number() == 0xd4c3b2a1;
    }

    fn magic_number(&self) -> u32 {
        bytes_to_u32(self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3], false)
    }

    fn major_version(&self) -> u16 {
        bytes_to_u16(self.bytes[4], self.bytes[5], self.is_swapped())
    }

    fn minor_version(&self) -> u16 {
        bytes_to_u16(self.bytes[6], self.bytes[7], self.is_swapped())
    }

    pub fn version(&self) -> String {
       format!("{}.{}", self.major_version(), self.minor_version()).to_string()
    }

    fn snap_len(&self) -> u32 {
        bytes_to_u32(self.bytes[16], self.bytes[17], self.bytes[18], self.bytes[19], self.is_swapped())
    }

    fn link_type(&self) -> u16 {
        if self.is_swapped() {
            bytes_to_u16(self.bytes[20], self.bytes[21], self.is_swapped())
        } else {
            bytes_to_u16(self.bytes[22], self.bytes[23], self.is_swapped())
        }
    }

    fn fcs(&self) -> u8 {
        if self.is_swapped() {
            return self.bytes[23].l_nibble();
        } else {
            return self.bytes[20].l_nibble();
        }
    }
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Magic: {}\nVersion: {}\nSnap Length: {}\nLink: {}\nFCS: {}\n",
            self.magic_number(),
            self.version(),
            self.snap_len(),
            LinkType::new(self.link_type()).to_string(),
            self.fcs(),
        )
    }
}

#[derive(Clone)]
pub struct PacketHeader {
    data: [u8;16],
    is_swapped: bool
}

impl fmt::Display for PacketHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TS upper: {}\nTS lower: {}\nCapture Len: {}\nOriginal Len: {}",
            self.ts_sec(),
            self.ts_ms_or_ns(),
            self.cap_len(),
            self.packet_len(),
        )
    }
}

impl PacketHeader {
    
    fn new(data: [u8;16], is_swapped: bool) -> Self {
        return Self{data, is_swapped};
    }

    fn ts_sec(&self) -> u32 {
        bytes_to_u32(self.data[0], self.data[1], self.data[2], self.data[3], self.is_swapped)
    }

    fn ts_ms_or_ns(&self) -> u32 {
        bytes_to_u32(self.data[4], self.data[5], self.data[6], self.data[7], self.is_swapped)
    }

    fn cap_len(&self) -> u32 {
        bytes_to_u32(self.data[8], self.data[9], self.data[10], self.data[11], self.is_swapped)
    }

    fn packet_len(&self) -> u32 {
        bytes_to_u32(self.data[12], self.data[13], self.data[14], self.data[15], self.is_swapped)
    }
}

#[derive(Clone)]
pub struct Packet {
    header: PacketHeader,
    data: Vec<u8>
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes: String = "".to_string();
        for i in 0..self.data.len() {
            bytes = format!("{} {:02X?}", bytes,  self.data[i])
        }
        write!(
            f,
            "{}\n{}\n",
            self.header,
            bytes,
        )
    }
}

impl Packet {

    pub fn new(header: PacketHeader, data: Vec<u8>) -> Self {
        return Self{header, data}; 
    }
}

enum ParserState {
    Header,
    Body
}

pub struct PacketParser {
    state: ParserState,
    active_header: [u8;16],
    active_data: Vec<u8>,
    curr_offset: usize,
    curr_pos: usize,
    curr_body_len: usize,
    packets: Vec<Packet>,
    is_swapped: bool,
}


impl PacketParser {

    pub fn new(swapped: bool) -> Self {
        return PacketParser{
            state: ParserState::Header,
            active_header: [0;16],
            active_data: vec![],
            curr_offset: 0,
            curr_pos: 0,
            curr_body_len: 0,
            packets: vec![],
            is_swapped: swapped,
        };
    }

    fn header_check(&mut self) {
        if self.active_header.len() == 16 && self.curr_pos == 16 {
            self.state = ParserState::Body;
            self.curr_pos = 0;
            let tmph = PacketHeader::new(self.active_header, self.is_swapped);
            self.curr_body_len = tmph.cap_len() as usize;
        }
    }

    fn body_check(&mut self) {
        if  self.curr_pos == self.curr_body_len {
            self.packets.push(
                Packet::new(PacketHeader::new(
                        self.active_header, self.is_swapped),
                        self.active_data.to_vec()
                )
            );
            self.state = ParserState::Header;
            self.active_header = [0;16];
            self.active_data = vec![];
            self.curr_pos = 0;
        }
    }

    fn put_byte(&mut self, byte: u8) {
        match self.state {
            ParserState::Header => self.active_header[self.curr_pos] = byte,
            ParserState::Body => self.active_data.push(byte),
        }
        self.curr_pos += 1;
        self.curr_offset += 1;
    }

    fn check_switch(&mut self) {
        match self.state {
            ParserState::Header => self.header_check(),
            ParserState::Body => self.body_check(),
        }
    }

    pub fn parse_packets(&mut self, data: Vec<u8>, offset: usize) -> Vec<Packet> {
        let bytes = get_vec_from_offset(data, offset);
        for byte in bytes.into_iter() {
            self.check_switch();
            self.put_byte(byte);
        }
        return self.packets.to_vec();
    }
}

fn get_vec_from_offset(v: Vec<u8>, n: usize) -> Vec<u8> {
    let mut c: usize = 0;
    let mut res: Vec<u8> = vec![];
    let _: Vec<()> =  v.into_iter().map(|x| {
        if c >= n {
            res.push(x)
        }
        c += 1;
    }).collect();
    return res;
}


#[cfg(test)]
mod tests {
    use super::*;


}
