use byte::Byte;

pub trait Row {
    fn as_u32(&self) -> u32;
    fn l_half_u16(&self) -> u16;
    fn r_half_u16(&self) -> u16;
    fn swapped_copy(&self) -> Self;
}

impl Row for [u8;4] {
    
    fn as_u32(&self) -> u32 {
        return self[0].to_u32(self[1], self[2], self[3]);
    }

    fn l_half_u16(&self) -> u16 {
        return self[0].to_u16(self[1]);
    }

    fn r_half_u16(&self) -> u16 {
        return self[2].to_u16(self[3]);
    }

    fn swapped_copy(&self) -> Self {
        return [self[3], self[2], self[1], self[0]]
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
}
