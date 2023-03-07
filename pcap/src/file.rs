use core::fmt;

use byte::Byte;

pub enum MagicNumber {
    PCAP,
    SWAPPED,
    PCAPNG,
    UNKNOWN,
}

impl MagicNumber {

    pub fn from_row(bytes: u32) -> Self {
        match bytes {
            0xa1b2c3d4 => Self::PCAP,
            0xd4c3b2a1 => Self::SWAPPED,
            0x0a0d0d0a => Self::PCAPNG,
            _ => Self::UNKNOWN,
        }
    }

    pub fn is_swapped(&self) -> bool {
        match self {
            Self::PCAP => false,
            Self::SWAPPED => true,
            Self::PCAPNG => false,
            Self::UNKNOWN => false,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::PCAP => "PCAP".to_string(),
            Self::SWAPPED => "PCAP (swapped)".to_string(),
            Self::PCAPNG => "PCAPNG (not supported)".to_string(),
            Self::UNKNOWN => "Invalid Magic Number".to_string(),
        }
    }
}

pub enum LinkType {
    NULL,
    ETHERNET,
    EXPETHERNET,
    AX25,
    PRONET,
    CHAOS,
    UNKNOWN,
}

impl LinkType {

    pub fn new(bytes: u16) -> Self {
        match bytes {
            0 => LinkType::NULL,
            1 => LinkType::ETHERNET,
            2 => LinkType::EXPETHERNET,
            3 => LinkType::AX25,
            4 => LinkType::PRONET,
            5 => LinkType::CHAOS,
            _ => LinkType::UNKNOWN,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            LinkType::NULL => "Null",
            LinkType::ETHERNET => "Ethernet",
            LinkType::EXPETHERNET => "Experimental Ethernet",
            LinkType::AX25 => "AX 25",
            LinkType::PRONET => "ProNET TokenRing",
            LinkType::CHAOS => "Chaos",
            LinkType::UNKNOWN => "Unknown",
        }
    }
    
}

pub struct FileHeader {
    magic_num: MagicNumber,
    major_ver: u16,
    minor_ver: u16,
    snap_len: u32,
    fcs: u8,
    link_type: LinkType,
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File Type: {}\nVersion: {}\nSnap Length: {}\nLinkType: {}\nFCS Length: {}\n",
            self.magic_num.to_string(),
            self.version(),
            self.snap_len,
            self.link_type.to_string(),
            self.fcs_len(),
        )
    }
}

impl FileHeader {
    
    pub fn new(mn: MagicNumber, mv: u16, miv: u16, snap_len: u32, fcs: u8, link: u16) -> Self {
        return Self{
            magic_num: mn,
            major_ver: mv,
            minor_ver: miv,
            snap_len,
            fcs,
            link_type: LinkType::new(link),
        };
    }

    pub fn fcs_len(&self) -> usize {
        let mut result: usize = 0;
        if self.fcs.nth_bit_set(0) {
            result += self.fcs as usize;
        }
        return result;
    }

    pub fn version(&self) -> String {
        return format!("{}.{}", self.major_ver, self.minor_ver);
    }

    pub fn swapped(&self) -> bool {
        return self.magic_num.is_swapped();
    }
}
