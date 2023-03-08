use std::fmt;
use byte::bytes_to_u16;

#[derive(Clone)]
pub struct MacAddress {
    bytes: [u8;6]
}

impl MacAddress {
    
    pub fn new(bytes: [u8;6]) -> Self {
        return Self { bytes }
    }

    pub fn empty() -> Self {
        return Self{ bytes: [0;6]  };
    }

    pub fn set_octet(&mut self, b: u8, idx: usize) {
        if idx > 5 {
            println!("Invalid Mac Address octet position {}", idx)
        } else {
            self.bytes[idx] = b
        }
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02X?}:{:02X?}:{:02X?}:{:02X?}:{:02X?}:{:02X?}",
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5],
        )
    }
}

pub enum PacketType {
    IPv4,
    IPv6,
    ARP,
    IPX,
    LENGTH(u16),
    UNKNWON
}

impl PacketType {

    pub fn new(bytes: u16) -> Self {
        if bytes <= 1500 {
            return PacketType::LENGTH(bytes);
        } 
        match bytes {
            0x0800 => PacketType::IPv4,
            0x0806 => PacketType::ARP,
            0x8137 => PacketType::IPX,
            0x86dd => PacketType::IPv6,
            _ => PacketType::UNKNWON,
        }
    }
}

impl fmt::Display for PacketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketType::IPv4 => write!(f, "IPv4"),
            PacketType::ARP => write!(f, "ARP"),
            PacketType::IPX => write!(f, "IPX"),
            PacketType::IPv6 => write!(f, "IPv6"),
            PacketType::LENGTH(len) => write!(f, "IEEE 802.3 ({} Bytes)", len),
            PacketType::UNKNWON => write!(f, "Unknown"),
        }
    }
}

pub struct EthernetFrame {
    dest: MacAddress,
    src: MacAddress,
    p_type: PacketType,
}

impl EthernetFrame {

    pub fn new(dest: MacAddress, src: MacAddress, p_type: PacketType) -> Self {
        return Self{
            dest,
            src,
            p_type
        };
    }

    pub fn is_802_3(&self) -> bool {
        match self.p_type {
            PacketType::LENGTH(_) => true,
            PacketType::IPv4 => false,
            PacketType::IPv6 => false,
            PacketType::IPX => false,
            PacketType::UNKNWON => false,
            PacketType::ARP => false,
        }
    }
}

impl fmt::Display for EthernetFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Destination: {}\nSource: {}\nType: {}\n", self.dest, self.src, self.p_type)
    }
}

enum ParserState {
    DEST,
    SRC,
    T,
    END,
}

impl ParserState {
    
    fn step(&self, pos: usize) -> Self {
        if pos < 6 {
            ParserState::DEST
        } else if pos < 12 {
            ParserState::SRC
        } else if pos <= 13 {
            ParserState::T
        } else {
            ParserState::END
        }
    } 
}

pub struct EthernetFrameParser {
    dest: MacAddress,
    src: MacAddress,
    p_type: [u8;2],
    curr_pos: usize,
    state: ParserState,
}

impl EthernetFrameParser {

    pub fn new() -> Self {
        return Self{
            dest: MacAddress::empty(),
            src: MacAddress::empty(),
            p_type: [0;2],
            curr_pos: 0,
            state: ParserState::DEST,
        };
    }

    fn put_d_byte(&mut self, b: u8) {
        self.dest.set_octet(b, self.curr_pos);
        self.curr_pos += 1;
    }

    fn put_s_byte(&mut self, b: u8) {
        let tmp = self.curr_pos - 6;
        self.src.set_octet(b, tmp);
        self.curr_pos += 1;
    }

    fn put_t_byte(&mut self, b: u8) {
        self.p_type[self.curr_pos - 12] = b;
        self.curr_pos += 1;
    }

    pub fn put_byte(&mut self, byte: u8) {
        self.state = self.state.step(self.curr_pos);

        match self.state {
           ParserState::DEST => self.put_d_byte(byte),
           ParserState::SRC => self.put_s_byte(byte),
           ParserState::T => self.put_t_byte(byte),
           ParserState::END => println!("Parser is done.")
        }
    }

    pub fn parse(&self) -> EthernetFrame {
        return EthernetFrame::new(
            self.dest.clone(),
            self.src.clone(),
            PacketType::new(
                bytes_to_u16(
                    self.p_type[0],
                    self.p_type[1],
                    false,
                )
            )
        );
    }
}

