use byte::{ByteStream, Byte};

use crate::formatter::Row;

pub struct RowParser {
    stream: ByteStream,
    loaded_row: [u8;4],
}

impl RowParser {
    
    pub fn new(stream: ByteStream) -> Self {
        return Self{stream, loaded_row: [0;4]};
    }

    pub fn get_nth_row(&self, n: usize, swapped: bool) -> Result<[u8;4], &'static str> {
        if n > self.len_rows() {
            return Err("Row out of bound.")
        } else {
            let start = n * 4;
            let end = start + 4;
            let mut result: [u8;4] = [0;4];
            let mut pos = 0;
            let data = self.stream.as_vec(swapped);
            for i in start..end {
                let item: u8 = data[i];
                result[pos] = item;
                pos += 1;
            }
            return Ok(result);
        }
    }

    pub fn load_row(&mut self, n: usize, swapped: bool) {
        self.loaded_row = self.get_nth_row(n, swapped).unwrap();
    }

    pub fn len_rows(&self) -> usize {
        return self.stream.len() / 4;
    }

    pub fn loaded_as_u32(&self) -> u32 {
        return self.loaded_row.as_u32();
    }

    pub fn loaded_l_half(&self) -> u16 {
        return self.loaded_row.l_half_u16();
    }

    pub fn loaded_r_half(&self) -> u16 {
        return self.loaded_row.r_half_u16();
    }

    pub fn get_nth_loaded_byte(&self, n: usize) -> u8 {
        if n <= 3 {
            return self.loaded_row[n];
        } else {
            panic!("Invalid index to take nth item of loaded row")
        }
    }

    pub fn l_nib_loaded_nth(&self, n: usize ) -> u8 {
        return self.get_nth_loaded_byte(n).l_nibble();
    }

    pub fn r_nib_loaded_nth(&self, n: usize ) -> u8 {
        return self.get_nth_loaded_byte(n).r_nibble();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_parser() {
        let stream = ByteStream::from_vec(
            vec![
                0xA1, 0xA1, 0xA1, 0xA1,
                0xB1, 0xB1, 0xB1, 0xB1,
                0xC1, 0xC1, 0xC1, 0xC1,
                0xD1, 0xD1, 0xD1, 0xD1,
        ]);
        let mut parser = RowParser::new(stream);
        parser.load_row(0, false);
        assert_eq!(parser.len_rows(), 4);
        assert_eq!(
            parser.loaded_as_u32(), 0xA1A1A1A1
        );
        parser.load_row(0, true);
        assert_eq!(parser.loaded_as_u32(), 0xD1D1D1D1);
        assert_eq!(parser.r_nib_loaded_nth(0), 1);
        assert_eq!(parser.l_nib_loaded_nth(0), 13);
    }
}
