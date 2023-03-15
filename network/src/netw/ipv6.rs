use std::{net::Ipv6Addr, fmt::Display};

use byte::ByteParser;

use crate::transport::protocols::TransportProtocol;

pub struct Ipv6AddressParser {
    buf: [u16;8],
    pos: usize,
}

impl Ipv6AddressParser {
    
    fn new() -> Self {
        return Self { buf: [0;8], pos: 0 }
    }

    fn done(&self) -> bool {
        self.pos == 8
    }

    fn clear(&mut self) {
        self.buf = [0;8];
        self.pos = 0;
    }

    fn set_bytes(&mut self, b: u16) {
        if !self.done() {
            self.buf[self.pos] = b;
            self.pos += 1;
        } else {
            println!("IPv6 Address Parser Buffer full");
        }
    }

    fn get_address(&self) -> Ipv6Addr {
        Ipv6Addr::new(
            self.buf[0],
            self.buf[1],
            self.buf[2],
            self.buf[3],
            self.buf[4], 
            self.buf[5],
            self.buf[6],
            self.buf[7]
        )
    }
}

pub enum IPv6HeaderField {
    V(u8),
    FLOW(u32),
    LEN(u16),
    PRT(TransportProtocol),
    HOPL(u8),
    SRC(Ipv6Addr),
    DST(Ipv6Addr),
    UNSET,
} 

impl Display for IPv6HeaderField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V(b) => write!(f, "IPv{}", b),
            Self::FLOW(b) => write!(f, "Flow: {:#010x}", b),
            Self::LEN(b) => write!(f, "Length: {}", b),
            Self::PRT(b) => write!(f, "Next Header: {}", b.to_str()),
            Self::HOPL(b) => write!(f, "Hop Length: {}", b),
            Self::SRC(b) => write!(f, "Source: {}", b),
            Self::DST(b) => write!(f, "Destination: {}", b),
            Self::UNSET => write!(f, "UNSET IPv6 Header Field"),
        }
    }
}

impl Clone for IPv6HeaderField {
    fn clone(&self) -> Self {
        match self {
            Self::V(b) => Self::V(*b),
            Self::FLOW(b) => Self::FLOW(*b),
            Self::LEN(b) => Self::LEN(*b),
            Self::PRT(b) => Self::PRT(b.clone()),
            Self::HOPL(b) => Self::HOPL(*b),
            Self::SRC(b) => Self::SRC(*b),
            Self::DST(b) => Self::DST(*b),
            Self::UNSET => Self::UNSET,
        }
    }
}

#[derive(Clone)]
pub struct IPv6Header {
    version: IPv6HeaderField,
    flow: IPv6HeaderField,
    length: IPv6HeaderField,
    proto: IPv6HeaderField,
    hop_len: IPv6HeaderField,
    source: IPv6HeaderField,
    destination: IPv6HeaderField,
}

impl Display for IPv6Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            self.version,
            self.flow,
            self.length,
            self.proto,
            self.hop_len,
            self.source,
            self.destination,
            )
    }
}

impl IPv6Header {

    pub fn new() -> Self {
        return Self{
            version: IPv6HeaderField::UNSET,
            flow: IPv6HeaderField::UNSET,
            length: IPv6HeaderField::UNSET,
            proto: IPv6HeaderField::UNSET,
            hop_len: IPv6HeaderField::UNSET,
            source: IPv6HeaderField::UNSET,
            destination: IPv6HeaderField::UNSET,
        };
    }

    pub fn set_field(&mut self, field: IPv6HeaderField) {
        match field {
            IPv6HeaderField::V(_) => self.version = field,
            IPv6HeaderField::FLOW(_) => self.flow = field,
            IPv6HeaderField::LEN(_) => self.length = field,
            IPv6HeaderField::PRT(_) => self.proto = field,
            IPv6HeaderField::HOPL(_) => self.hop_len = field,
            IPv6HeaderField::SRC(_) => self.source = field,
            IPv6HeaderField::DST(_) => self.destination = field,
            IPv6HeaderField::UNSET => println!("Cant set unset field in Ipv6 header")
        }
    }
}

pub struct IPv6HeaderParser {
    b_parser: ByteParser,
    a_parser: Ipv6AddressParser,
    curr_field: IPv6HeaderField,
    header: IPv6Header,
}

impl IPv6HeaderParser {
    
    pub fn new(swap: bool) -> Self {
        return Self{
            b_parser: ByteParser::new(swap),
            a_parser: Ipv6AddressParser::new(),
            curr_field: IPv6HeaderField::V(0),
            header: IPv6Header::new()
        };
    }

    pub fn parse(&mut self, b: u8) {
        self.step(b);
        match self.curr_field {
            IPv6HeaderField::V(_) => self.version(),
            IPv6HeaderField::FLOW(_) => self.flow(),
            IPv6HeaderField::LEN(_) => self.length(),
            IPv6HeaderField::PRT(_) => self.proto(),
            IPv6HeaderField::HOPL(_) => self.hop_len(),
            IPv6HeaderField::SRC(_) => self.source(),
            IPv6HeaderField::DST(_) => self.destination(),
            IPv6HeaderField::UNSET => println!("IPv6 header parsing complete, cant set byte"),
        }
    }

    pub fn get_header(&self) -> IPv6Header {
        return self.header.clone();
    }

    fn step(&mut self, b: u8) {
        match self.curr_field {
            IPv6HeaderField::V(_)|
                IPv6HeaderField::PRT(_)|
                IPv6HeaderField::HOPL(_) 
                => self.b_parser.set_word(b),
            IPv6HeaderField::LEN(_)|
                IPv6HeaderField::SRC(_)|
                IPv6HeaderField::DST(_)
                => self.b_parser.set_d_byte(b),
            IPv6HeaderField::FLOW(_) => self.b_parser.set_q_byte(b),
            IPv6HeaderField::UNSET => println!("Cant set byte for unset IPv6 Header field"),
        }
    }

    fn version(&mut self) {
        self.header.set_field(
            IPv6HeaderField::V(self.b_parser.word_l_nibble())
        );
        self.b_parser.set_q_byte(self.b_parser.word_r_nibble());
        self.curr_field = IPv6HeaderField::FLOW(0);
    }

    fn flow(&mut self) {
        if self.b_parser.qword_done() {
            self.header.set_field(
                IPv6HeaderField::FLOW(self.b_parser.qword_as_u32())
            );
            self.b_parser.reset_qword();
            self.curr_field = IPv6HeaderField::LEN(0);
        }
    }

    fn length(&mut self) {
        if self.b_parser.dword_done() {
            self.header.set_field(
                IPv6HeaderField::LEN(self.b_parser.dword_as_u16())
            );
            self.b_parser.reset_dword();
            self.curr_field = IPv6HeaderField::PRT(TransportProtocol::UNKNOWN(254));
        }
    }

    fn proto(&mut self) {
        self.header.set_field(IPv6HeaderField::PRT(TransportProtocol::new(self.b_parser.word())));
        self.curr_field = IPv6HeaderField::HOPL(0);
    }

    fn hop_len(&mut self) {
        self.header.set_field(IPv6HeaderField::HOPL(self.b_parser.word()));
        self.curr_field = IPv6HeaderField::SRC(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    }

    fn source(&mut self) {
        if self.b_parser.dword_done() {
            self.a_parser.set_bytes(self.b_parser.dword_as_u16());
            self.b_parser.reset_dword();
        }
        if self.a_parser.done() {
            self.header.set_field(
                IPv6HeaderField::SRC(self.a_parser.get_address())
            );
            self.a_parser.clear();
            self.curr_field = IPv6HeaderField::DST(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        }
    }
    
    fn destination(&mut self) {
        if self.b_parser.dword_done() {
            self.a_parser.set_bytes(self.b_parser.dword_as_u16());
            self.b_parser.reset_dword();
        }
        if self.a_parser.done() {
            self.header.set_field(
                IPv6HeaderField::DST(self.a_parser.get_address())
            );
            self.a_parser.clear();
            self.curr_field = IPv6HeaderField::UNSET;
        }
    }
}


