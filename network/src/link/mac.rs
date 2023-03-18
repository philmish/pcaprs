use std::fmt::Display;

pub struct MacAddress {
    bytes: [u8;6],
}

impl Clone for MacAddress {
    fn clone(&self) -> Self {
        MacAddress::new(self.bytes)
    }
}

impl MacAddress {

    pub fn empty() -> Self {
        Self{bytes: [0;6]}
    }

    pub fn new(bytes: [u8;6]) -> Self {
        Self{bytes}
    }

    pub fn to_string(&self) -> String {
        format!(
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

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct MacAddressParser {
    buf: [u8;6],
    pos: usize,
}

impl MacAddressParser {
    
    pub fn new() -> Self {
        Self{
            buf: [0;6],
            pos: 0,
        }
    }

    pub fn done(&self) -> bool {
        self.pos == 6
    }

    pub fn set_byte(&mut self, b: u8) {
        if self.done() {
            println!("Mac Address buffer full");
        } else {
            self.buf[self.pos] = b;
            self.pos += 1;
        }
    }

    pub fn get_adress(&self) -> MacAddress {
        MacAddress::new(self.buf)
    }

    pub fn reset(&mut self) {
        self.buf = [0;6];
        self.pos = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_address() {
        let empty = MacAddress::empty();
        assert_eq!("00:00:00:00:00:00".to_string(), empty.to_string());

        let addr = MacAddress::new([0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x00]);
        assert_eq!("DE:AD:BE:EF:00:00".to_string(), addr.to_string());
    }

    #[test]
    fn test_mac_address_parser() {
        let mut parser = MacAddressParser::new();
        let b = [0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x00];

        for i in b {
            parser.set_byte(i);
        }
        assert!(parser.done());
        let addr = MacAddress::new([0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x00]);
        assert_eq!(addr.to_string(), parser.get_adress().to_string());
        parser.reset();
        assert!(!parser.done());
        assert_eq!("00:00:00:00:00:00", parser.get_adress().to_string());
    }

}
