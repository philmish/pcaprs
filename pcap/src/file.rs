use core::fmt;
use std::fmt::Display;

use byte::{Byte, bytes_to_u32, bytes_to_u16};

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
}

impl Display for MagicNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::PCAP => "PCAP".to_string(),
            Self::SWAPPED => "PCAP (swapped)".to_string(),
            Self::PCAPNG => "PCAPNG (not supported)".to_string(),
            Self::UNKNOWN => "Invalid Magic Number".to_string(),
        };
        write!(f, "{}", string)
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
    bytes: [u8;24],
}

impl FileHeader {
    
    pub fn new(bytes: Vec<u8>) -> Result<Self, &'static str> {
        if bytes.len() < 24 {
            return Err("Insufficent data length to parse header.");
        }
        let mut data: [u8;24] = [0;24];
        let mut c: usize = 0;
        let _: Vec<()> = bytes.iter().map(|x| {
            if c < 24 {
                data[c] = *x
            }
            c += 1;
        }).collect();
        Ok(Self { bytes: data })
    }

    pub fn is_swapped(&self) -> bool {
        self.magic_number() == 0xd4c3b2a1
    }

    fn magic_number(&self) -> u32 {
        bytes_to_u32(self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3], false)
    }

    fn major_version(&self) -> u16 {
        bytes_to_u16(self.bytes[4], self.bytes[5], self.is_swapped())
    }

    fn minor_version(&self) -> u16 {
        bytes_to_u16(self.bytes[6], self.bytes[7], self.is_swapped())
    }

    pub fn version(&self) -> String {
       format!("{}.{}", self.major_version(), self.minor_version())
    }

    fn snap_len(&self) -> u32 {
        bytes_to_u32(self.bytes[16], self.bytes[17], self.bytes[18], self.bytes[19], self.is_swapped())
    }

    fn link_type(&self) -> u16 {
        if self.is_swapped() {
            bytes_to_u16(self.bytes[20], self.bytes[21], self.is_swapped())
        } else {
            bytes_to_u16(self.bytes[22], self.bytes[23], self.is_swapped())
        }
    }

    fn fcs(&self) -> u8 {
        if self.is_swapped() {
            self.bytes[23].l_nibble()
        } else {
            self.bytes[20].l_nibble()
        }
    }
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Magic: {}\nVersion: {}\nSnap Length: {}\nLink: {}\nFCS: {}\n",
            MagicNumber::from_row(self.magic_number()),
            self.version(),
            self.snap_len(),
            LinkType::new(self.link_type()).to_string(),
            self.fcs(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_number_swapped() {
        let mn = MagicNumber::from_row(0xD4C3B2A1);
        assert!(mn.is_swapped());
    }

    #[test]
    fn test_file_header() {
        let b = vec![
            0xD4, 0xC3, 0xB2, 0xA1,
            0x02, 0x00, 0x04, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF,
            0x01, 0x01, 0xFF, 0xFF,
        ];

        let fh = FileHeader::new(b).unwrap();
        assert!(fh.is_swapped());
        assert_eq!(fh.major_version(), 2);
        assert_eq!(fh.minor_version(), 4);
        assert_eq!(fh.snap_len(), 0xFFFFFFFF);

    }

}

