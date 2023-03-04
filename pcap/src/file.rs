use core::fmt;

enum MagicNumber {
    PCAP,
    SWAPPED,
    PCAPNG,
    UNKNOWN,
}

impl MagicNumber {

    fn from_row(bytes: u32) -> Self {
        match bytes {
            0xa1b2c3d4 => Self::PCAP,
            0xd4c3b2a1 => Self::SWAPPED,
            0x0a0d0d0a => Self::PCAPNG,
            _ => Self::UNKNOWN,
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

pub struct FileHeader {
    magic_num: MagicNumber,
    major_ver: u16,
    minor_ver: u16,
    snap_len: u32,
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File Type: {}\nVersion: {}\nSnap Length: {}\n",
            self.magic_num.to_string(),
            self.version(),
            self.snap_len,
        )
    }
}

impl FileHeader {
    
    pub fn new(mn: u32, mv: u16, miv: u16, snap_len: u32) -> Self {
        return Self{
            magic_num: MagicNumber::from_row(mn),
            major_ver: mv,
            minor_ver: miv,
            snap_len,
        };
    }

    pub fn version(&self) -> String {
        return format!("{}.{}", self.major_ver, self.minor_ver);
    }
}
