use byte::Byte;

pub trait Row {
    fn as_u32(&self) -> u32;
    fn l_half_u16(&self) -> u16;
    fn r_half_u16(&self) -> u16;
    fn swapped_copy(&self) -> Self;
}

impl Row for [u8;4] {
    
    fn as_u32(&self) -> u32 {
        self[0].to_u32(self[1], self[2], self[3])
    }

    fn l_half_u16(&self) -> u16 {
        self[0].to_u16(self[1])
    }

    fn r_half_u16(&self) -> u16 {
        self[2].to_u16(self[3])
    }

    fn swapped_copy(&self) -> Self {
        [self[3], self[2], self[1], self[0]]
    }
}

pub struct RowParser {
    stream: Vec<u8>,
    loaded_row: [u8;4],
}

impl RowParser {
    
    pub fn new(stream: Vec<u8>) -> Self {
        Self{stream, loaded_row: [0;4]}
    }

    pub fn range_to_bytestream(&self, start: usize, end: usize) -> Result<Vec<u8>, &'static str> {
        if start > end || end > self.len_rows() {
            return Err("Invalid range.")
        }
        let mut result: Vec<u8> = vec![];
        let mut tmp_row: [u8;4];
        for i in start..end + 1 {
            tmp_row = self.get_nth_row(i).unwrap();
            for j in 0..4 {
                result.push(tmp_row[j]);
            }
        }
        Ok(result)
    }

    #[allow(clippy::explicit_counter_loop)]
    pub fn get_nth_row(&self, n: usize) -> Result<[u8;4], &'static str> {
        if n > self.len_rows() {
            Err("Row out of bound.")
        } else {
            let start = n * 4;
            let end = start + 4;
            let mut result: [u8;4] = [0;4];
            let mut pos = 0;
            for i in start..end {
                let item: u8 = self.stream[i];
                result[pos] = item;
                pos += 1;
            }
            Ok(result)
        }
    }

    pub fn load_row(&mut self, n: usize, swapped: bool) { 
        self.loaded_row = self.get_nth_row(n).unwrap();
        if swapped {
            self.loaded_row = self.loaded_row.swapped_copy()
        }
    }

    pub fn len_rows(&self) -> usize {
        self.stream.len() / 4
    }

    pub fn loaded_as_u32(&self) -> u32 {
        self.loaded_row.as_u32()
    }

    pub fn loaded_l_half(&self) -> u16 {
        self.loaded_row.l_half_u16()
    }

    pub fn loaded_r_half(&self) -> u16 {
        self.loaded_row.r_half_u16()
    }

    pub fn get_nth_loaded_byte(&self, n: usize) -> u8 {
        if n <= 3 {
            self.loaded_row[n]
        } else {
            panic!("Invalid index to take nth item of loaded row")
        }
    }

    pub fn l_nib_loaded_nth(&self, n: usize ) -> u8 {
        self.get_nth_loaded_byte(n).l_nibble()
    }

    pub fn r_nib_loaded_nth(&self, n: usize ) -> u8 {
        self.get_nth_loaded_byte(n).r_nibble()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_as_u32() {
        let cases: &[([u8;4], u32)] = &[
            ([0xA1, 0xB2, 0xC3, 0xD4], 0xA1B2C3D4)
        ];

        for c in cases {
            assert_eq!(c.0.as_u32(), c.1);
        }
    }

    #[test]
    fn test_halfs_as_u16() {
        let l_cases: &[([u8;4], u16, u16)] = &[
            ([7,10,1,1], 0x070A, 0x0101),
        ];

        for cl in l_cases {
            assert_eq!(cl.0.l_half_u16(), cl.1);
            assert_eq!(cl.0.r_half_u16(), cl.2);
        }
    }

    #[test]
    fn test_row_parser() {
        let stream: Vec<u8> = vec![
            0xA1, 0xA1, 0xA1, 0xA1,
            0xB1, 0xB1, 0xB1, 0xB1,
            0xC1, 0xC1, 0xC1, 0xC1,
            0xD1, 0xD1, 0xD1, 0xD1,
        ];
        let mut parser = RowParser::new(stream);
        parser.load_row(0, false);
        assert_eq!(parser.len_rows(), 4);
        assert_eq!(
            parser.loaded_as_u32(), 0xA1A1A1A1
        );
        parser.load_row(0, true);
        assert_eq!(parser.loaded_as_u32(), 0xA1A1A1A1);
        assert_eq!(parser.r_nib_loaded_nth(0), 1);
        assert_eq!(parser.l_nib_loaded_nth(0), 10);

        parser.load_row(1, false);
        assert_eq!(parser.loaded_l_half(), 0xB1B1);
        assert_eq!(parser.loaded_r_half(), 0xB1B1);
    }

    #[test]
    #[should_panic]
    fn test_loaded_byte_out_of_bounds_panics() {
        let stream: Vec<u8> = vec![
            0xA1, 0xA1, 0xA1, 0xA1,
            0xB1, 0xB1, 0xB1, 0xB1,
            0xC1, 0xC1, 0xC1, 0xC1,
            0xD1, 0xD1, 0xD1, 0xD1,
        ];
        let mut parser = RowParser::new(stream);
        parser.load_row(0, false);
        parser.get_nth_loaded_byte(5);
    }
}
