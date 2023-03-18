use std::{net::Ipv4Addr, fmt::Display};

use byte::ByteParser;

use super::mac::{MacAddress, MacAddressParser};

pub enum ARPHeaderField {
    MACT(u16),
    PROTT(u16),
    HWS(u8),
    PAS(u8),
    OP(u16),
    SRCMAC(MacAddress),
    SRCIP(Ipv4Addr),
    DSTMAC(MacAddress),
    DSTIP(Ipv4Addr),
    UNSET,
}

impl Display for ARPHeaderField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MACT(b) => write!(f, "Hw addr type: {}", b),
            Self::PROTT(b) => write!(f, "Protocol addr type: {}", b),
            Self::HWS(b) => write!(f, "Hardware addr size: {}", b),
            Self::PAS(b) => write!(f, "Protocol addr size: {}", b),
            Self::OP(b) => write!(f, "Operation: {}", b),
            Self::SRCMAC(b) => write!(f, "Src Mac: {}", b),
            Self::SRCIP(b) => write!(f, "Src IP: {}", b),
            Self::DSTMAC(b) => write!(f, "Dest Mac: {}", b),
            Self::DSTIP(b) => write!(f, "Dest IP: {}", b),
            Self::UNSET => write!(f, "ARP Field UNSET"),
        }
    }
}

impl Clone for ARPHeaderField {
    fn clone(&self) -> Self {
        match self {
            Self::MACT(b) => Self::MACT(*b),
            Self::PROTT(b) => Self::PROTT(*b),
            Self::HWS(b) => Self::HWS(*b),
            Self::PAS(b) => Self::PAS(*b),
            Self::OP(b) => Self::OP(*b),
            Self::SRCMAC(b) => Self::SRCMAC(b.clone()),
            Self::SRCIP(b) => Self::SRCIP(*b),
            Self::DSTMAC(b) => Self::DSTMAC(b.clone()),
            Self::DSTIP(b) => Self::DSTIP(*b),
            Self::UNSET => Self::UNSET,
        }
    }
}

#[derive(Clone)]
pub struct ARPHeader {
    mac_type: ARPHeaderField,
    proto_type: ARPHeaderField,
    hardware_addr_s: ARPHeaderField,
    proto_addr_s: ARPHeaderField,
    operation: ARPHeaderField,
    src_mac: ARPHeaderField,
    src_ip: ARPHeaderField,
    dst_mac: ARPHeaderField,
    dst_ip: ARPHeaderField,
}

impl ARPHeader {
    
    pub fn new() -> Self {
        Self { 
            mac_type: ARPHeaderField::UNSET, 
            proto_type: ARPHeaderField::UNSET,
            hardware_addr_s: ARPHeaderField::UNSET,
            proto_addr_s: ARPHeaderField::UNSET,
            operation: ARPHeaderField::UNSET,
            src_mac: ARPHeaderField::UNSET,
            src_ip: ARPHeaderField::UNSET,
            dst_mac: ARPHeaderField::UNSET,
            dst_ip: ARPHeaderField::UNSET,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            self.mac_type,
            self.proto_type,
            self.hardware_addr_s,
            self.proto_addr_s,
            self.operation,
            self.src_mac,
            self.src_ip,
            self.dst_mac,
            self.dst_ip,
        )
    }

    pub fn set_field(&mut self, field: ARPHeaderField) {
        match field {
            ARPHeaderField::MACT(_) => self.mac_type = field,
            ARPHeaderField::PROTT(_) => self.proto_type = field,
            ARPHeaderField::HWS(_) => self.hardware_addr_s = field,
            ARPHeaderField::PAS(_) => self.proto_addr_s = field,
            ARPHeaderField::OP(_) => self.operation = field,
            ARPHeaderField::SRCMAC(_) => self.src_mac = field,
            ARPHeaderField::SRCIP(_) => self.src_ip = field,
            ARPHeaderField::DSTMAC(_) => self.dst_mac = field,
            ARPHeaderField::DSTIP(_) => self.dst_ip = field,
            ARPHeaderField::UNSET => println!("Cant set unset ARP Header field"),
        }
    }

    pub fn get_field(&mut self, field: ARPHeaderField) -> ARPHeaderField {
        match field {
            ARPHeaderField::MACT(_) => self.mac_type.clone(),
            ARPHeaderField::PROTT(_) => self.proto_type.clone(),
            ARPHeaderField::HWS(_) => self.hardware_addr_s.clone(),
            ARPHeaderField::PAS(_) => self.proto_addr_s.clone(),
            ARPHeaderField::OP(_) => self.operation.clone(),
            ARPHeaderField::SRCMAC(_) => self.src_mac.clone(),
            ARPHeaderField::SRCIP(_) => self.src_ip.clone(),
            ARPHeaderField::DSTMAC(_) => self.dst_mac.clone(),
            ARPHeaderField::DSTIP(_) => self.dst_ip.clone(),
            ARPHeaderField::UNSET => ARPHeaderField::UNSET,
        }

    }
}

impl Display for ARPHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct ARPHeaderParser {
    b_parser: ByteParser,
    m_parser: MacAddressParser,
    curr_field: ARPHeaderField,
    header: ARPHeader,
}

impl ARPHeaderParser {

    pub fn new(swap: bool) -> Self {
        Self {
            b_parser: ByteParser::new(swap),
            m_parser: MacAddressParser::new(), 
            curr_field: ARPHeaderField::MACT(0),
            header: ARPHeader::new(),
        }
    }

    pub fn parse(&mut self, b: u8) {
        self.step(b);
        match self.curr_field {
            ARPHeaderField::MACT(_) => self.mac_type(),
            ARPHeaderField::PROTT(_) => self.proto_type(),
            ARPHeaderField::HWS(_) => self.hardware_addr_s(),
            ARPHeaderField::PAS(_) => self.proto_addr_s(),
            ARPHeaderField::OP(_) => self.operation(),
            ARPHeaderField::SRCMAC(_) => self.src_mac(),
            ARPHeaderField::SRCIP(_) => self.src_ip(),
            ARPHeaderField::DSTMAC(_) => self.dst_mac(),
            ARPHeaderField::DSTIP(_) => self.dst_ip(),
            ARPHeaderField::UNSET => println!("Cant parse byte into unset arp header field"),
        }
    }

    fn step(&mut self, b: u8) {
        match self.curr_field {
            ARPHeaderField::MACT(_)|
                ARPHeaderField::PROTT(_)|
                ARPHeaderField::OP(_)
                => self.b_parser.set_d_byte(b),
            ARPHeaderField::HWS(_)|
                ARPHeaderField::PAS(_)
                => self.b_parser.set_word(b),
            ARPHeaderField::DSTIP(_)|
                ARPHeaderField::SRCIP(_)
                => self.b_parser.set_q_byte(b),
            ARPHeaderField::DSTMAC(_)|
                ARPHeaderField::SRCMAC(_)
                => self.m_parser.set_byte(b),
            ARPHeaderField::UNSET => println!("Cant step into unset ARP header field"),
        }
    }

    pub fn get_header(&self) -> ARPHeader {
        self.header.clone()
    }

    fn mac_type(&mut self) {
        if self.b_parser.dword_done() {
            self.header.set_field(
                ARPHeaderField::MACT(self.b_parser.dword_as_u16())
            );
            self.b_parser.reset_dword();
            self.curr_field = ARPHeaderField::PROTT(0);
        }
    }

    fn proto_type(&mut self) {
        if self.b_parser.dword_done() {
            self.header.set_field(
                ARPHeaderField::PROTT(self.b_parser.dword_as_u16())
            );
            self.b_parser.reset_dword();
            self.curr_field = ARPHeaderField::HWS(0);
        }
    }

    fn hardware_addr_s(&mut self) {
        self.header.set_field(ARPHeaderField::HWS(self.b_parser.word()));
        self.curr_field = ARPHeaderField::PAS(0);
    }

    fn proto_addr_s(&mut self) {
        self.header.set_field(ARPHeaderField::PAS(self.b_parser.word()));
        self.curr_field = ARPHeaderField::OP(0);
    }

    fn operation(&mut self) {
        if self.b_parser.dword_done() {
            self.header.set_field(
                ARPHeaderField::OP(self.b_parser.dword_as_u16())
            );
            self.b_parser.reset_dword();
            self.curr_field = ARPHeaderField::SRCMAC(MacAddress::empty());
        }
    }

    fn src_mac(&mut self) {
        if self.m_parser.done() {
            self.header.set_field(ARPHeaderField::SRCMAC(self.m_parser.get_adress()));
            self.m_parser.reset();
            self.curr_field = ARPHeaderField::SRCIP(Ipv4Addr::new(0, 0, 0, 0));
        }
    }

    fn src_ip(&mut self) {
        if self.b_parser.qword_done() {
            self.header.set_field(
                ARPHeaderField::SRCIP(self.b_parser.qword_as_ipv4())
            );
            self.b_parser.reset_qword();
            self.curr_field = ARPHeaderField::DSTMAC(MacAddress::empty());
        }
    }

    fn dst_mac(&mut self) {
        if self.m_parser.done() {
            self.header.set_field(ARPHeaderField::DSTMAC(self.m_parser.get_adress()));
            self.m_parser.reset();
            self.curr_field = ARPHeaderField::DSTIP(Ipv4Addr::new(0, 0, 0, 0));
        }
    }

    fn dst_ip(&mut self) {
        if self.b_parser.qword_done() {
            self.header.set_field(
                ARPHeaderField::DSTIP(self.b_parser.qword_as_ipv4())
            );
            self.b_parser.reset_qword();
            self.curr_field = ARPHeaderField::UNSET;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_header_field() {
        let mut header = ARPHeader::new();
        header.set_field(ARPHeaderField::MACT(0xA1B2));
        assert!(
            matches!(
                header.get_field(ARPHeaderField::MACT(0)),
                ARPHeaderField::MACT(0xA1B2)
            )
        );

        header.set_field(ARPHeaderField::PROTT(0xA1B2));
        assert!(
            matches!(
                header.get_field(ARPHeaderField::PROTT(0)),
                ARPHeaderField::PROTT(0xA1B2)
            )
        );

    }
}
