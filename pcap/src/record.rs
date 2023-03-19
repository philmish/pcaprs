use std::fmt;
use byte::bytes_to_u32;
use network::{
    ethernet_frame::{EthernetFrame, EthernetFrameParser, PacketType},
    ip::{IPv4Header, IPv4HeaderParser},
    transport::udp::{UdpHeader, UdpHeaderParser},
    transport::tcp::{TcpHeader, TcpHeaderParser},
    netw::ipv6::{IPv6Header, IPv6HeaderParser}, link::arp::{ARPHeader, ARPHeaderParser}
};


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
        Self{data, is_swapped}
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
        let frame = self.parse_ethernet_frame();
        if frame.is_arp() {
            return write!(f, "{}\n{}\n{}", self.header, frame, self.parse_arp())
        }
        let ip = self.ip_header_to_string(frame.packet_type());
        let mut bytes: String = "".to_string();
        for i in 0..self.data.len() {
            bytes = format!("{} {:02X?}", bytes,  self.data[i])
        }
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}",
            self.header,
            frame,
            ip,
            self.parse_tcp_header(),
            bytes,
        )
    }
}

impl Record {

    pub fn new(header: RecordHeader, data: Vec<u8>) -> Self {
        Self{header, data} 
    }

    pub fn parse_ethernet_frame(&self) -> EthernetFrame {
        let mut parser = EthernetFrameParser::new();
        for i in 0..14 {
            parser.put_byte(self.data[i])
        }
        parser.parse()
    }

    pub fn parse_arp(&self) -> ARPHeader {
        let mut parser = ARPHeaderParser::new(false);
        for i in 14..42 {
            parser.parse(self.data[i])
        }
        parser.get_header()
    }

    pub fn parse_ipv4_header(&self) -> IPv4Header {
        let mut parser = IPv4HeaderParser::new(true);
        parser.step(self.data[14]);
        for i in 15..34 {
            parser.step(self.data[i])
        }
        parser.get_header()
    }

    pub fn parse_ipv6_header(&self) -> IPv6Header {
        let mut parser = IPv6HeaderParser::new(true);
        for i in 14..54 {
            parser.parse(self.data[i])
        }
        parser.get_header()
    }

    pub fn ip_header_to_string(&self, t: PacketType) -> String {
        match t {
            PacketType::IPv4 => format!("{}", self.parse_ipv4_header()),
            PacketType::IPv6 => format!("{}", self.parse_ipv6_header()),
            PacketType::ARP => "ARP Header parsing not implemented".to_string(),
            PacketType::IPX => "IPX Header parsing not implemented".to_string(),
            PacketType::LENGTH(b) => format!("IEEE 802.3 Header parsing not implemented (length: {})", b),
            PacketType::UNKNWON => "UNKNWON ip header type encountered".to_string(),

        }
    }

    pub fn parse_udp_header(&self) -> UdpHeader {
        let mut parser = UdpHeaderParser::new(false);
        for i in 34..43 {
            parser.parse(self.data[i])
        }
        parser.get_header()
    }

    pub fn parse_tcp_header(&self) -> TcpHeader {
        let mut parser = TcpHeaderParser::new(false);
        for i in  34..54 {
            parser.parse(self.data[i])
        }
        parser.get_header()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_header() {
        let bytes: [u8;16] = [
            0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0xAA,
            0x00, 0x00, 0x00, 0xAA,
        ];
        let unswapped_rh = RecordHeader::new(bytes, false);
        assert_eq!(unswapped_rh.ts_sec(), 0x00000001);
        assert_eq!(unswapped_rh.ts_ms_or_ns(), 0x00000002);
        assert_eq!(unswapped_rh.cap_len(), 0x000000AA);
        assert_eq!(unswapped_rh.packet_len(), 0x000000AA);
    }
}

