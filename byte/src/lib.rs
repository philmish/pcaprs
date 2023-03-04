use std::collections::VecDeque;

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

pub struct ByteStream {
    data: VecDeque<u8>
}

impl ByteStream {
    
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        return Self{data: bytes.into()};
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn as_vec(&self, swapped: bool) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        let mut clone = self.data.clone();
        if !swapped {
            result = clone.into();
        } else {
            for _ in 0..self.data.len() {
                match clone.pop_back() {
                    Some(b) => result.push(b),
                    None => panic!("Tried to push None value from Byte stream to vec"),
                }
            }
        }
        return result;
    }

    pub fn vec_from_offset(&self, offset: usize, swapped: bool) -> Vec<u8> {
        let mut pos: usize  = 0;
        let mut result: VecDeque<u8> = VecDeque::with_capacity(self.len() - offset);
        let _: Vec<()> = self.as_vec(swapped).into_iter().map(|x| {
            if pos >= offset {
                result.push_front(x);
            }
            pos += 1;
        }).collect();
        return result.into();
    }

    pub fn stream_from_offset(&self, offset: usize, swapped: bool) -> Self {
        return Self::from_vec(self.vec_from_offset(offset, swapped));
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
}
