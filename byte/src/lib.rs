use std::net::Ipv4Addr;

pub trait Byte {
    fn to_u16(&self, b: Self) -> u16;
    fn to_u32(&self, b: Self, c: Self, d: Self) -> u32;
    fn l_nibble(&self) -> u8;
    fn r_nibble(&self) -> u8;
    fn nth_bit_set(&self, pos: u8) -> bool;
}

impl Byte for u8 {

    fn to_u16(&self, b: Self) -> u16 {
        return ((*self as u16) << 8) | b as u16;
    }

    fn to_u32(&self, b: Self, c: Self, d: Self) -> u32 {
        return ((*self as u32) << 24) +
        ((b as u32) << 16) +
        ((c as u32) << 8) +
        ((d as u32) << 0)
    }

    fn r_nibble(&self) -> u8 {
        return self & 0b1111;
    }

    fn l_nibble(&self) -> u8 {
        return self >> 4;
    }

    fn nth_bit_set(&self, pos: u8) -> bool {
        for n in 0..8 {
            if n == pos {
                return 1 == self >> n & 1
            }
        }
        return false;
    }
}

pub fn bytes_to_u32(a: u8, b: u8, c: u8, d: u8, swapped: bool) -> u32 {
    if swapped {
        ((d as u32) << 24) +
        ((c as u32) << 16) +
        ((b as u32) << 8) +
        ((a as u32) << 0)
    } else {
        ((a as u32) << 24) +
        ((b as u32) << 16) +
        ((c as u32) << 8) +
        ((d as u32) << 0)
    }
}

pub fn bytes_to_u16(a: u8, b: u8, swapped: bool) -> u16 {
    if swapped {
        ((b as u16) << 8) | a as u16
    } else {
        ((a as u16) << 8) | b as u16
    }
}


pub struct ByteParser {
    curr_word: u8,
    curr_dword: [u8;2],
    d_pos: usize,
    curr_qword: [u8;4],
    q_pos: usize,
    b_swap: bool,
}

impl ByteParser {

    pub fn new(b_swap: bool) -> Self {
        return Self{
            curr_word: 0,
            curr_dword: [0;2],
            d_pos: 0,
            curr_qword: [0;4],
            q_pos: 0,
            b_swap,
        }
    }

    pub fn word(&self) -> u8 {
        return self.curr_word.clone();
    }

    pub fn set_word(&mut self, b: u8) {
        self.curr_word = b;
    }

    pub fn word_l_nibble(&self) -> u8 {
        return self.curr_word.l_nibble();
    }

    pub fn word_r_nibble(&self) -> u8 {
        return self.curr_word.r_nibble();
    }

    pub fn toggle_swap(&mut self) {
        self.b_swap = !self.b_swap;
    }

    pub fn dword_done(&self) -> bool {
        self.d_pos == 2
    }

    pub fn dword_as_u16(&self) -> u16 {
        return bytes_to_u16(self.curr_dword[0], self.curr_dword[1], self.b_swap);
    }

    pub fn qword(&self) -> [u8;4] {
        let tmp = self.curr_qword.clone();
        if self.b_swap {
            return [tmp[3],tmp[2],tmp[1], tmp[0]];
        } else {
            return tmp;
        }
    }

    pub fn qword_as_u32(&self) -> u32 {
        return bytes_to_u32(
            self.curr_qword[0],
            self.curr_qword[1],
            self.curr_qword[2],
            self.curr_qword[3],
            self.b_swap
        );
    }

    pub fn qword_as_ipv4(&self) -> Ipv4Addr {
        let tmp = self.qword();
        return Ipv4Addr::new(tmp[0], tmp[1], tmp[2], tmp[3]);
    }

    pub fn qword_done(&self) -> bool {
        self.q_pos == 4
    }

    pub fn set_q_byte(&mut self, b: u8) {
        if self.q_pos >= 4 {
            println!("QWord full.");
            return;
        } else {
            self.curr_qword[self.q_pos] = b;
            self.q_pos += 1;
            return;
        }
    }

    pub fn reset_qword(&mut self) {
        self.curr_qword = [0;4];
        self.q_pos = 0;
    }

    pub fn set_d_byte(&mut self, b: u8) {
        if self.d_pos >= 2 {
            println!("DWord full.");
            return;
        } else {
            self.curr_dword[self.d_pos] = b;
            self.d_pos += 1;
            return;
        }
    }

    pub fn reset_dword(&mut self) {
        self.curr_dword = [0;2];
        self.d_pos = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u16() {
        let right: u8 = 0xA1;
        let cases: &[(u8, u16)] = &[
            (0xCE, 0xA1CE),
            (0x01, 0xA101),
        ];

        for c in cases {
            assert_eq!(right.to_u16(c.0), c.1);
        }
    }

    #[test]
    fn test_to_u32() {
        let right: u8 = 0xA1;
        let cases: &[(u8, u8, u8, u32)] = &[
            (0xA1, 0xA1, 0xA1, 0xA1A1A1A1),
            (0, 0, 1, 0xA1000001),
        ];

        for c in cases {
            assert_eq!(right.to_u32(c.0, c.1, c.2), c.3);
        }
    }

    #[test]
    fn test_l_nibble() {
        let cases: &[(u8, u8)] = &[
            (4, 0),
            (0b0101_0111, 0b0000_0101),
            (0xD1, 13),
        ];

        for c in cases {
            assert_eq!(c.0.l_nibble(), c.1);
        }
    }

    #[test]
    fn test_r_nibble() {
        let cases: &[(u8, u8)] = &[
            (0xA0, 0x0),
            (0b0101_0111, 0b0000_0111),
        ];

        for c in cases {
            assert_eq!(c.0.r_nibble(), c.1);
        }
    }

    #[test]
    fn test_nth_bit_set() {
        let cases: &[(u8, u8, bool)] = &[
            (0b1000_0000, 7, true),
            (0x0A, 1, true),
            (0x0A, 3, true),
        ];

        for c in cases {
            assert_eq!(c.0.nth_bit_set(c.1), c.2);
        }
    }

    #[test]
    fn test_bytes_to_u16() {
        let cases: &[(u8, u8, bool, u16)] = &[
            (0xA1, 0xC3, false, 0xA1C3),
            (0xA1, 0xC3, true, 0xC3A1),
        ];

        for c in cases {
            assert_eq!(bytes_to_u16(c.0, c.1, c.2), c.3);
        }
    }

    #[test]
    fn test_bytes_to_u32() {
        let cases: &[(u8, u8, u8, u8, bool, u32)] = &[
            (0xA1, 0xB2, 0xC3, 0xD4, false, 0xA1B2C3D4),
            (0xA1, 0xB2, 0xC3, 0xD4, true, 0xD4C3B2A1),
        ];

        for c in cases {
            assert_eq!(bytes_to_u32(c.0, c.1, c.2, c.3, c.4), c.5);
        }
    }

    #[test]
    fn test_set_word_for_parser() {
        let mut parser = ByteParser::new(false);
        parser.set_word(1);
        assert_eq!(parser.word(), 1);
    }

    #[test]
    fn test_get_word_nibbles() {
        let cases: &[(u8, u8, u8)] = &[
            (0xA1, 0x0A, 0x01),
        ];

        let mut parser = ByteParser::new(false);
        for c in cases {
           parser.set_word(c.0);
           assert_eq!(c.1, parser.word_l_nibble());
           assert_eq!(c.2, parser.word_r_nibble());
        }
    }

    #[test]
    fn test_set_d_word_as_u16() {
        let cases: &[(u8, u8, bool, u16)] = &[
            (0xA1, 0xB2, false, 0xA1B2),
            (0xA1, 0xB2, true, 0xB2A1),
        ];

        for c in cases {
            let mut parser = ByteParser::new(c.2);
            parser.set_d_byte(c.0);
            parser.set_d_byte(c.1);
            assert!(parser.dword_done());
            assert_eq!(c.3, parser.dword_as_u16());
            parser.reset_dword();
            assert_eq!(0, parser.dword_as_u16());
        }
    }

    #[test]
    fn test_set_q_word_as_u32() {
        let cases: &[(u8, u8, u8, u8, bool, u32)] = &[
            (0xA1, 0xB2, 0xC3, 0xD4, false, 0xA1B2C3D4),
            (0xA1, 0xB2, 0xC3, 0xD4, true, 0xD4C3B2A1),
        ];

        for c in cases {
            let mut parser = ByteParser::new(c.4);
            parser.set_q_byte(c.0);
            parser.set_q_byte(c.1);
            parser.set_q_byte(c.2);
            parser.set_q_byte(c.3);
            assert!(parser.qword_done());
            assert_eq!(c.5, parser.qword_as_u32());
            parser.reset_qword();
            assert_eq!(0, parser.qword_as_u32());
        }
    }

    #[test]
    fn test_q_word_as_ipv4() {
        let cases: &[(u8, u8, u8, u8, bool, Ipv4Addr)] = &[
            (192, 168, 0, 1, false, Ipv4Addr::new(192, 168, 0, 1)),
            (1, 0, 168, 192, true, Ipv4Addr::new(192, 168, 0, 1))
        ];

        for c in cases {
            let mut parser = ByteParser::new(c.4);
            parser.set_q_byte(c.0);
            parser.set_q_byte(c.1);
            parser.set_q_byte(c.2);
            parser.set_q_byte(c.3);
            assert_eq!(c.5, parser.qword_as_ipv4());
        }
        
    }
}
