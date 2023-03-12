use std::fmt::Display;
use std::net::Ipv4Addr;

use byte::ByteParser;

pub enum IPv4HeaderField {
    V(u8),
    IHL(u8),
    TOS(u8),
    LEN(u16),
    ID(u16),
    FF(u16),
    TTL(u8),
    PRT(IpDataProtocol),
    CHECK(u16),
    SRC(Ipv4Addr),
    DST(Ipv4Addr),
    UNSET,
}

impl Clone for IPv4HeaderField {

    fn clone(&self) -> Self {
        match self {
            IPv4HeaderField::V(b) => IPv4HeaderField::V(*b),
            IPv4HeaderField::IHL(b) => IPv4HeaderField::IHL(*b),
            IPv4HeaderField::TOS(b) => IPv4HeaderField::TOS(*b),
            IPv4HeaderField::LEN(b) => IPv4HeaderField::LEN(*b),
            IPv4HeaderField::ID(b) => IPv4HeaderField::ID(*b),
            IPv4HeaderField::FF(b) => IPv4HeaderField::FF(*b),
            IPv4HeaderField::TTL(b) => IPv4HeaderField::TTL(*b),
            IPv4HeaderField::PRT(b) => IPv4HeaderField::PRT(b.clone()),
            IPv4HeaderField::CHECK(b) => IPv4HeaderField::CHECK(*b),
            IPv4HeaderField::SRC(b) => IPv4HeaderField::SRC(*b),
            IPv4HeaderField::DST(b) =>IPv4HeaderField::DST(*b),
            IPv4HeaderField::UNSET => IPv4HeaderField::UNSET,
        }
    }
}

impl Display for IPv4HeaderField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IPv4HeaderField::V(b) => write!(f, "IP Version: {}", b),
            IPv4HeaderField::IHL(b) => write!(f, "IP Header Len: {}", b),
            IPv4HeaderField::TOS(b) => write!(f, "ToS: {:#04b}", b),
            IPv4HeaderField::LEN(b) => write!(f, "Packet Len: {}", b),
            IPv4HeaderField::ID(b) => write!(f, "ID: {:#06x}", b),
            IPv4HeaderField::FF(b) => write!(f, "FF: {:#018b}", b),
            IPv4HeaderField::TTL(b) => write!(f, "TTL: {}", b),
            IPv4HeaderField::PRT(b) => write!(f, "Protocol: {}", b.to_str()),
            IPv4HeaderField::CHECK(b) => write!(f, "Checksum: {:#018b}", b),
            IPv4HeaderField::SRC(b) => write!(f, "Source: {}", b),
            IPv4HeaderField::DST(b) => write!(f, "Destination: {}", b),
            IPv4HeaderField::UNSET => write!(f, "UNSET"),
        }
    }
}

pub enum IpDataProtocol {
    IPv6HopByHop,
    ICMP,
    IGMP,
    GGP,
    IPinIP,
    ST,
    TCP,
    CBT,
    UDP,
    EGP,
    IGP,
    NVP2,
    UNKNOWN(u8),
}

impl IpDataProtocol {

    pub fn new(b: u8) -> Self {
        match b {
            0 => Self::IPv6HopByHop,
            1 => Self::ICMP,
            2 => Self::IGMP,
            3 => Self::GGP,
            4 => Self::IPinIP,
            5 => Self::ST,
            6 => Self::TCP,
            7 => Self::CBT,
            8 => Self::EGP,
            9 => Self::IGP,
            11 => Self::NVP2,
            17 => Self::UDP,
            _ => Self::UNKNOWN(b),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Self::IPv6HopByHop => "IPv6 Hop-By-Hop Option".to_string(),
            Self::ICMP => "ICMP".to_string(),
            Self::IGMP => "Internet Group Management Protocol".to_string(),
            Self::GGP => "Gateway-to-Gateway".to_string(),
            Self::IPinIP => "IP in IP (encapsulated)".to_string(),
            Self::ST => "Internet Stream Protocol".to_string(),
            Self::TCP => "TCP".to_string(),
            Self::CBT => "Core Based Trees".to_string(),
            Self::EGP => "Exterior Gateway".to_string(),
            Self::IGP => "Interior Gateway".to_string(),
            Self::NVP2 => "Network Voice Protocol".to_string(),
            Self::UDP => "UDP".to_string(),
            Self::UNKNOWN(b) => format!("Unknown {}", b),
        }
    }
}

impl Display for IpDataProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Protocol: {}", self.to_string())
    }
}

impl Clone for IpDataProtocol {
    fn clone(&self) -> Self {
        match self {
            Self::IPv6HopByHop => Self::IPv6HopByHop,
            Self::ICMP => Self::ICMP,
            Self::IGMP => Self::IGMP,
            Self::GGP => Self::GGP,
            Self::IPinIP => Self::IPinIP,
            Self::ST => Self::ST,
            Self::TCP => Self::TCP,
            Self::CBT => Self::CBT,
            Self::EGP => Self::EGP,
            Self::IGP => Self::IGP,
            Self::NVP2 => Self::NVP2,
            Self::UDP => Self::UDP,
            Self::UNKNOWN(b) => Self::UNKNOWN(*b),
        }
    }
}

#[derive(Clone)]
pub struct IPv4Header {
    version: IPv4HeaderField,
    ihl: IPv4HeaderField,
    tos: IPv4HeaderField,
    length: IPv4HeaderField,
    id: IPv4HeaderField,
    ff: IPv4HeaderField,
    ttl: IPv4HeaderField,
    proto: IPv4HeaderField,
    checksum: IPv4HeaderField,
    src: IPv4HeaderField,
    dst: IPv4HeaderField
}

impl IPv4Header {
    pub fn empty() -> Self {
        Self {
            version: IPv4HeaderField::UNSET,
            ihl: IPv4HeaderField::UNSET,
            tos: IPv4HeaderField::UNSET,
            length: IPv4HeaderField::UNSET,
            id: IPv4HeaderField::UNSET,
            ff: IPv4HeaderField::UNSET,
            ttl: IPv4HeaderField::UNSET,
            proto: IPv4HeaderField::UNSET,
            checksum: IPv4HeaderField::UNSET,
            src: IPv4HeaderField::UNSET,
            dst: IPv4HeaderField::UNSET, 
        }
    }

    pub fn set_field(&mut self, field: IPv4HeaderField) {
        match field {
            IPv4HeaderField::V(_) => self.version = field.clone(),
            IPv4HeaderField::IHL(_) => self.ihl = field.clone(),
            IPv4HeaderField::TOS(_) => self.tos = field.clone(),
            IPv4HeaderField::LEN(_) => self.length = field.clone(),
            IPv4HeaderField::ID(_) => self.id = field.clone(),
            IPv4HeaderField::FF(_) => self.ff = field.clone(),
            IPv4HeaderField::TTL(_) => self.ttl = field,
            IPv4HeaderField::PRT(_) => self.proto = field,
            IPv4HeaderField::CHECK(_) => self.checksum = field,
            IPv4HeaderField::SRC(_) => self.src = field,
            IPv4HeaderField::DST(_) => self.dst = field,
            IPv4HeaderField::UNSET => return,
        }
    }
}

impl Display for IPv4Header {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            self.version,
            self.ihl,
            self.tos,
            self.length,
            self.id,
            self.ff,
            self.ttl,
            self.proto,
            self.checksum,
            self.src,
            self.dst,
        )
    }
}

pub struct IPv4HeaderParser {
    parser: ByteParser,
    curr_state: IPv4HeaderField,
    header: IPv4Header,
}

impl IPv4HeaderParser {

    pub fn new(b_swap: bool) -> Self {
        return Self{
            parser: ByteParser::new(b_swap),
            curr_state: IPv4HeaderField::V(0),
            header: IPv4Header::empty(),
        };
    }

    fn set_byte(&mut self, b: u8) {
        match self.curr_state {
            IPv4HeaderField::IHL(_) => println!(""),
            IPv4HeaderField::V(_)|
                IPv4HeaderField::TOS(_)|
                IPv4HeaderField::TTL(_)|
                IPv4HeaderField::PRT(_)
                => self.parser.set_word(b),
            IPv4HeaderField::LEN(_)|
                IPv4HeaderField::ID(_)|
                IPv4HeaderField::CHECK(_)|
                IPv4HeaderField::FF(_)
                => self.parser.set_d_byte(b),
            IPv4HeaderField::DST(_)| 
                IPv4HeaderField::SRC(_)
                => self.parser.set_q_byte(b),
            IPv4HeaderField::UNSET 
                => println!("Cant set unset field with byte {}", b),
        }
    }

    pub fn step(&mut self, b: u8) {
        self.set_byte(b);
        match self.curr_state {
            IPv4HeaderField::V(_) => self.version(),
            IPv4HeaderField::IHL(_) => return,
            IPv4HeaderField::TOS(_) => self.tos(),
            IPv4HeaderField::LEN(_) => self.length(),
            IPv4HeaderField::ID(_) => self.id(),
            IPv4HeaderField::TTL(_) => self.ttl(),
            IPv4HeaderField::FF(_) => self.ff(),
            IPv4HeaderField::PRT(_) => self.proto(),
            IPv4HeaderField::CHECK(_) => self.checksum(),
            IPv4HeaderField::SRC(_) => self.src(),
            IPv4HeaderField::DST(_) => self.dst(),
            IPv4HeaderField::UNSET => println!("Cant step in unset field with byte {}", b),
        }
    }

    fn version(&mut self) {
        self.header.set_field(
            IPv4HeaderField::V(self.parser.word_l_nibble())
        );
        self.header.set_field(
            IPv4HeaderField::IHL(self.parser.word_r_nibble())
        );
        self.curr_state = IPv4HeaderField::TOS(0);
    }

    fn tos(&mut self) {
        self.header.set_field(
            IPv4HeaderField::TOS(self.parser.word())
        );
        self.curr_state = IPv4HeaderField::LEN(0);
    }

    fn length(&mut self) {
        if self.parser.dword_done() {
            self.parser.toggle_swap();
            self.header.set_field(
                IPv4HeaderField::LEN(self.parser.dword_as_u16())
            );
            self.parser.toggle_swap();
            self.parser.reset_dword();
            self.curr_state = IPv4HeaderField::ID(0);
        }
    }

    fn id(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                IPv4HeaderField::ID(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_state = IPv4HeaderField::FF(0);
        }
    }

    fn ttl(&mut self) {
        self.header.set_field(
            IPv4HeaderField::TTL(self.parser.word())
        );
        self.curr_state = IPv4HeaderField::PRT(IpDataProtocol::UNKNOWN(254))
    }

    fn ff(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                IPv4HeaderField::FF(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_state = IPv4HeaderField::TTL(0);
        }
    }

    fn proto(&mut self) {
        self.header.set_field(
            IPv4HeaderField::PRT(IpDataProtocol::new(self.parser.word())),
        );
        self.curr_state = IPv4HeaderField::CHECK(0);
    }

    fn checksum(&mut self) {
        if self.parser.dword_done() {
            self.header.set_field(
                IPv4HeaderField::CHECK(self.parser.dword_as_u16())
            );
            self.parser.reset_dword();
            self.curr_state = IPv4HeaderField::SRC(Ipv4Addr::new(0, 0, 0, 0));
        }
    }

    fn src(&mut self) {
        if self.parser.qword_done() {
            self.header.set_field(
                IPv4HeaderField::SRC(self.parser.qword_as_ipv4())
            );
            self.parser.reset_qword();
            self.curr_state = IPv4HeaderField::DST(Ipv4Addr::new(0, 0, 0, 0));
        }
    }

    fn dst(&mut self) {
        if self.parser.qword_done() {
            self.header.set_field(
                IPv4HeaderField::DST(self.parser.qword_as_ipv4())
            );
            self.parser.reset_qword();
            self.curr_state = IPv4HeaderField::UNSET;
        }
    }

    pub fn get_header(&self) -> IPv4Header {
        return self.header.clone();
    }
}

