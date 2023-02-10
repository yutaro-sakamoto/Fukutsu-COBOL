#[cfg(test)]
pub mod move_tests {
    use super::super::data::*;
    #[test]
    pub fn move_alphanumeric() {
        let mut src = CobData {
            data: &mut vec![' ' as u8; 5],
            typ: CobFieldType::Alphanumeric,
            digits: 0,
            scale: 0,
            flags: FLAG_NONE,
            pic: "",
        };
        let mut dst = CobData {
            data: &mut vec![' ' as u8; 5],
            typ: CobFieldType::Alphanumeric,
            digits: 0,
            scale: 0,
            flags: FLAG_NONE,
            pic: "",
        };
        let test_data: &[u8] = &['h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8];
        for (i, c) in test_data.iter().enumerate() {
            src.data[i] = *c;
        }

        dst.move_from(&src);

        for (i, c) in test_data.iter().enumerate() {
            assert_eq!(dst.data[i], test_data[i]);
        }
    }
}
