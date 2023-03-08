use std::fmt;
use byte::bytes_to_u32;
use network::ethernet_frame::{EthernetFrame, EthernetFrameParser};


#[derive(Clone)]
pub struct RecordHeader {
    data: [u8;16],
    is_swapped: bool
}

impl fmt::Display for RecordHeader {
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

impl RecordHeader {
    
    pub fn new(data: [u8;16], is_swapped: bool) -> Self {
        return Self{data, is_swapped};
    }

    fn ts_sec(&self) -> u32 {
        bytes_to_u32(self.data[0], self.data[1], self.data[2], self.data[3], self.is_swapped)
    }

    fn ts_ms_or_ns(&self) -> u32 {
        bytes_to_u32(self.data[4], self.data[5], self.data[6], self.data[7], self.is_swapped)
    }

    pub fn cap_len(&self) -> u32 {
        bytes_to_u32(self.data[8], self.data[9], self.data[10], self.data[11], self.is_swapped)
    }

    fn packet_len(&self) -> u32 {
        bytes_to_u32(self.data[12], self.data[13], self.data[14], self.data[15], self.is_swapped)
    }
}

#[derive(Clone)]
pub struct Record {
    header: RecordHeader,
    data: Vec<u8>
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes: String = "".to_string();
        for i in 0..self.data.len() {
            bytes = format!("{} {:02X?}", bytes,  self.data[i])
        }
        write!(
            f,
            "{}\n{}\n{}\n",
            self.header,
            bytes,
            self.parse_ethernet_frame(),
        )
    }
}

impl Record {

    pub fn new(header: RecordHeader, data: Vec<u8>) -> Self {
        return Self{header, data}; 
    }

    pub fn parse_ethernet_frame(&self) -> EthernetFrame {
        let mut parser = EthernetFrameParser::new();
        for i in 0..14 {
            parser.put_byte(self.data[i])
        }
        return parser.parse();
    }
}
