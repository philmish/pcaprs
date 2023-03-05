use core::fmt;

use byte::ByteStream;

pub struct PacketHeader {
    ts_sec: u32,
    ts_low: u32,
    capture_len: u32,
    original_len: u32,
}

impl fmt::Display for PacketHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TS Sec: {}\nTS Low: {}\nCapLen: {}\nOrigLen: {}\n",
            self.ts_sec,
            self.ts_low,
            self.capture_len,
            self.original_len,
        )
    }
}

impl PacketHeader {
    
    pub fn new(tss: u32, tsl: u32, cap: u32, orig: u32) -> Self {
        return Self{
            ts_sec: tss,
            ts_low: tsl,
            capture_len: cap,
            original_len: orig,
        };
    }
}

pub struct Packet {
    header: PacketHeader,
    data: ByteStream,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.header)
    }
}

impl Packet {
    
    pub fn new(header: PacketHeader, data: Vec<u8>) -> Self {
        return Self{header, data: ByteStream::from_vec(data)}
    }

    pub fn get_data(&self, swapped: bool) -> Vec<u8> {
        return self.data.as_vec(swapped);
    }
}
