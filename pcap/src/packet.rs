use byte::ByteStream;

pub struct PacketHeader {
    ts_sec: u32,
    ts_low: u32,
    capture_len: u32,
    original_len: u32,
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
