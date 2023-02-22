#[cfg(test)]
mod move_tests {
    use super::super::data::*;
    #[test]
    pub fn move_alphanumeric() {
        let src = CobolField {
            start_index: 0,
            len: 5,
            typ: CobolFieldType::Alphanumeric,
            digits: 0,
            scale: 0,
            flags: FLAG_NONE,
            pic: "",
        };
        let dst = CobolField {
            start_index: 5,
            len: 5,
            typ: CobolFieldType::Alphanumeric,
            digits: 0,
            scale: 0,
            flags: FLAG_NONE,
            pic: "",
        };

        let initial_data = ['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'].map(|i| i as u8);
        let mut core: CobolCore = CobolCore::make_by_array(&initial_data);
        core.move_field(src, dst);
        assert_eq!(core.field_as_string(dst), "hello".to_string());
        assert_eq!(core.field_as_string(src), "hello".to_string());
    }
}
