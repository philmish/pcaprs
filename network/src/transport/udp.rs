use std::fmt::Display;

use byte::ByteParser;

pub enum UdpHeaderField {
    SRC(u16),
    DST(u16),
    LEN(u16),
    CHECK(u16),
    UNSET,
}

impl Display for UdpHeaderField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SRC(b) => write!(f, "Src Port: {}", b),
            Self::DST(b) => write!(f ,"Dest Port: {}", b),
            Self::LEN(b) => write!(f, "Length: {}", b),
            Self::CHECK(b) => write!(f, "Checksum: {:#018b}", b),
            Self::UNSET => write!(f, "UNSET")
        }
    }
}

impl Clone for UdpHeaderField {
    fn clone(&self) -> Self {
        match self {
            Self::SRC(b) => Self::SRC(*b),
            Self::DST(b) => Self::DST(*b),
            Self::LEN(b) => Self::LEN(*b),
            Self::CHECK(b) => Self::CHECK(*b),
            Self::UNSET => Self::UNSET,
        }
    }
}

#[derive(Clone)]
pub struct UdpHeader {
    src: UdpHeaderField,
    dst: UdpHeaderField,
    len: UdpHeaderField,
    check: UdpHeaderField,
}

impl Display for UdpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}\n{}", self.src, self.dst, self.len, self.check)
    }
}

impl UdpHeader {

    pub fn new() -> Self {
        return UdpHeader {
            src: UdpHeaderField::UNSET,
            dst: UdpHeaderField::UNSET, 
            len: UdpHeaderField::UNSET,
            check: UdpHeaderField::UNSET,
        }
    }

    pub fn set_field(&mut self, field: UdpHeaderField) {
        match field {
            UdpHeaderField::SRC(_) => self.src = field,
            UdpHeaderField::DST(_) => self.dst = field,
            UdpHeaderField::LEN(_) => self.len = field,
            UdpHeaderField::CHECK(_) => self.check = field,
            UdpHeaderField::UNSET => print!("Cant set unset UDP Header Field"),
        }
    }
}

pub struct UdpHeaderParser {
    parser: ByteParser,
    curr_field: UdpHeaderField,
    header: UdpHeader,
}

impl UdpHeaderParser {
    
    pub fn new(swap: bool) -> Self {
        return Self{
            parser: ByteParser::new(swap),
            curr_field: UdpHeaderField::SRC(0),
            header: UdpHeader::new(),
        };
    }

    pub fn parse(&mut self, b: u8) {
        self.step(b);
        match self.curr_field {
            UdpHeaderField::SRC(_) => self.src(),
            UdpHeaderField::DST(_) => self.dst(),
            UdpHeaderField::LEN(_) => self.len(),
            UdpHeaderField::CHECK(_) => self.check(),
            UdpHeaderField::UNSET => println!("Cant parse byte for unset udp header field"),
        }
    }

    pub fn get_header(&self) -> UdpHeader {
        self.header.clone()
    }

    fn step(&mut self, b: u8) {
        match self.curr_field {
            UdpHeaderField::SRC(_)|
                UdpHeaderField::DST(_)|
                UdpHeaderField::LEN(_)|
                UdpHeaderField::CHECK(_)
                => self.parser.set_d_byte(b),
            UdpHeaderField::UNSET => println!("Cant set byte to unset UDP Header field"),
        }
    }

    fn src(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                UdpHeaderField::SRC(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_field = UdpHeaderField::DST(0);
        }
    }

    fn dst(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                UdpHeaderField::DST(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_field = UdpHeaderField::LEN(0);
        }
    }

    fn len(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                UdpHeaderField::LEN(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_field = UdpHeaderField::CHECK(0);
        }
    }

    fn check(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                UdpHeaderField::CHECK(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_field = UdpHeaderField::UNSET;
        }
    }
}
