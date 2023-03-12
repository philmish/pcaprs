use std::fmt::Display;

use byte::ByteParser;

pub enum TcpHeaderField {
    SRC(u16),
    DST(u16),
    SEQ(u32),
    ACK(u32),
    HLEN(u8),
    FLAGS(u8),
    WSIZE(u16),
    CHECK(u16),
    UPOINT(u16),
    UNSET,
}

impl Clone for TcpHeaderField {
    fn clone(&self) -> Self {
        match self {
            Self::SRC(b) => Self::SRC(*b),
            Self::DST(b) => Self::DST(*b),
            Self::SEQ(b) => Self::SEQ(*b),
            Self::ACK(b) => Self::ACK(*b),
            Self::HLEN(b) => Self::HLEN(*b),
            Self::FLAGS(b) => Self::FLAGS(*b),
            Self::WSIZE(b) => Self::WSIZE(*b),
            Self::CHECK(b) => Self::CHECK(*b),
            Self::UPOINT(b) => Self::UPOINT(*b),
            Self::UNSET => Self::UNSET,
        }
    }
}

impl TcpHeaderField {

    pub fn to_string(&self) -> String {
        match self {
            Self::SRC(b) => format!("{}", b),
            Self::DST(b) => format!("{}", b),
            Self::SEQ(b) => format!("{}", b),
            Self::ACK(b) => format!("{}", b),
            Self::HLEN(b) => format!("{}", b),
            Self::FLAGS(b) => format!("{:#010b}", b),
            Self::WSIZE(b) => format!("{}", b),
            Self::CHECK(b) => format!("{}", b),
            Self::UPOINT(b) => format!("{}", b),
            Self::UNSET => format!("UNSET"),

        }
    }
}

#[derive(Clone)]
pub struct TcpHeader {
    src: TcpHeaderField,
    dst: TcpHeaderField,
    seq: TcpHeaderField,
    ack: TcpHeaderField,
    hlen: TcpHeaderField,
    flags: TcpHeaderField,
    wsize: TcpHeaderField,
    check: TcpHeaderField,
    upoint: TcpHeaderField
}

impl Display for TcpHeader {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Src Port: {}\nDest Port: {}\nSeq: {}\nACK: {}\nLen: {}\nFlags: {}\nWSize: {}\nCheck: {}\nUpoint: {}\n",
            self.src.to_string(),
            self.dst.to_string(),
            self.seq.to_string(),
            self.ack.to_string(),
            self.hlen.to_string(),
            self.flags.to_string(),
            self.wsize.to_string(),
            self.check.to_string(),
            self.upoint.to_string(),
        )
    }
}

impl TcpHeader {
    
    pub fn new() -> Self {
        return Self{
            src: TcpHeaderField::UNSET,
            dst: TcpHeaderField::UNSET,
            seq: TcpHeaderField::UNSET,
            ack: TcpHeaderField::UNSET,
            hlen: TcpHeaderField::UNSET,
            flags: TcpHeaderField::UNSET,
            wsize: TcpHeaderField::UNSET,
            check: TcpHeaderField::UNSET,
            upoint: TcpHeaderField::UNSET,
        };
    }

    pub fn set_field(&mut self, field: TcpHeaderField) {
        match field {
            TcpHeaderField::SRC(_) => self.src = field,
            TcpHeaderField::DST(_) => self.dst = field,
            TcpHeaderField::SEQ(_) => self.seq = field,
            TcpHeaderField::ACK(_) => self.ack = field,
            TcpHeaderField::HLEN(_) => self.hlen = field,
            TcpHeaderField::FLAGS(_) => self.flags = field,
            TcpHeaderField::WSIZE(_) => self.wsize = field,
            TcpHeaderField::CHECK(_) => self.check = field,
            TcpHeaderField::UPOINT(_) => self.upoint = field,
            TcpHeaderField::UNSET => println!("Cant set unset field"),
        }
    }
}

pub struct TcpHeaderParser {
    parser: ByteParser,
    curr_field: TcpHeaderField,
    header: TcpHeader,
}

impl TcpHeaderParser {
    
    pub fn new(_swap: bool) -> Self {
        return Self { 
            parser: ByteParser::new(false),
            curr_field: TcpHeaderField::SRC(0),
            header: TcpHeader::new()
        }
    }

    pub fn get_header(&self) -> TcpHeader {
        return self.header.clone();
    }

    fn step(&mut self, b: u8) {
        match self.curr_field {
            TcpHeaderField::SRC(_)|
                TcpHeaderField::DST(_)|
                TcpHeaderField::WSIZE(_)|
                TcpHeaderField::CHECK(_)|
                TcpHeaderField::UPOINT(_) 
                => self.parser.set_d_byte(b),
            TcpHeaderField::SEQ(_)|
                TcpHeaderField::ACK(_)
                => self.parser.set_q_byte(b),
            TcpHeaderField::HLEN(_)|
                TcpHeaderField::FLAGS(_)
                => self.parser.set_word(b),
            TcpHeaderField::UNSET => println!("Cant set unset byte"),
        }
    }

    pub fn parse(&mut self, b: u8) {
        self.step(b);
        match self.curr_field {
            TcpHeaderField::SRC(_) => self.src(),
            TcpHeaderField::DST(_) => self.dst(),
            TcpHeaderField::SEQ(_) => self.seq(),
            TcpHeaderField::ACK(_) => self.ack(),
            TcpHeaderField::HLEN(_) => self.hlen(),
            TcpHeaderField::FLAGS(_) => self.flags(),
            TcpHeaderField::WSIZE(_) => self.wsize(),
            TcpHeaderField::CHECK(_) => self.check(),
            TcpHeaderField::UPOINT(_) => self.upoint(),
            TcpHeaderField::UNSET => println!("Tcp Header full."),
        }
    }

    fn src(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                TcpHeaderField::SRC(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_field = TcpHeaderField::DST(0);
        }
    }

    fn dst(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                TcpHeaderField::DST(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_field = TcpHeaderField::SEQ(0);
        }
    }

    fn seq(&mut self) {
        if self.parser.qword_done() {
            self.header.set_field(
                TcpHeaderField::SEQ(self.parser.qword_as_u32())
            );
            self.parser.reset_qword();
            self.curr_field = TcpHeaderField::ACK(0);
        }
    }

    fn ack(&mut self) {
        if self.parser.qword_done() {
            self.header.set_field(
                TcpHeaderField::ACK(self.parser.qword_as_u32())
            );
            self.parser.reset_qword();
            self.curr_field = TcpHeaderField::HLEN(0)
        }
    }

    fn hlen(&mut self) {
        self.header.set_field(TcpHeaderField::HLEN(self.parser.word_l_nibble()));
        self.curr_field = TcpHeaderField::FLAGS(0);
    }

    fn flags(&mut self) {
        self.header.set_field(TcpHeaderField::FLAGS(self.parser.word()));
        self.curr_field = TcpHeaderField::WSIZE(0);
    }

    fn wsize(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                TcpHeaderField::WSIZE(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_field = TcpHeaderField::CHECK(0);
        }
    }

    fn check(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                TcpHeaderField::CHECK(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_field = TcpHeaderField::UPOINT(0);
        }
    }

    fn upoint(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                TcpHeaderField::UPOINT(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_field = TcpHeaderField::UNSET;
        }
    }
}
