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

