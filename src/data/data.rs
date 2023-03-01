use std::cmp::min;
use std::marker::Copy;
use std::str;
use wasm_bindgen::prelude::*;

/*pub struct DataStorage<'a> {
    pub arrays: Vec<&'a [u8]>,
    pub table: Vec<CobField<'a>>,
}*/
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct CobolCore {
    data: Vec<u8>,
    fields: Vec<CobolField>,
}

impl CobolCore {
    pub fn make_by_array(data: &[u8]) -> CobolCore {
        CobolCore {
            data: data.to_vec(),
            fields: Vec::new(),
        }
    }
}

#[wasm_bindgen]
impl CobolCore {
    pub fn new(data_size: usize) -> CobolCore {
        CobolCore {
            data: vec![0; data_size],
            fields: Vec::new(),
        }
    }

    pub fn register_field(
        &mut self,
        start_index: usize,
        len: usize,
        typ: CobolFieldType,
        digits: usize,
        scale: i64,
        flags: u8,
        pic: String,
    ) -> FieldId {
        self.fields.push(CobolField {
            start_index: start_index,
            len: len,
            typ: typ,
            digits: digits,
            scale: scale,
            flags: flags,
            pic: pic,
        });
        self.fields.len() - 1
    }

    pub fn move_field(&mut self, src_id: FieldId, dst_id: FieldId) -> bool {
        if let (Some(src), Some(dst)) = (self.fields.get(src_id), self.fields.get(dst_id)) {
            match src.typ {
                CobolFieldType::Alphanumeric => match src.typ {
                    CobolFieldType::Alphanumeric
                    | CobolFieldType::AlphanumericAll
                    | CobolFieldType::AlphanumericEdited
                    | CobolFieldType::National
                    | CobolFieldType::NationalAll
                    | CobolFieldType::NationalEdited => {
                        let m = min(src.len, dst.len);
                        for i in 0..m {
                            self.data[dst.start_index + i] = self.data[src.start_index + i];
                        }
                        for i in m..dst.len {
                            self.data[dst.start_index + i] = ' ' as u8;
                        }
                    }
                    _ => unreachable!("The unimplemented case of move From"),
                },
                _ => unreachable!("The unimplemented case of move From"),
            }
            true
        } else {
            false
        }
    }

    pub fn field_as_vec_u8(&self, field: CobolField) -> Vec<u8> {
        self.data[field.start_index..field.start_index + field.len].to_vec()
    }

    pub fn field_as_string(&self, field_id: FieldId) -> String {
        match self.fields.get(field_id) {
            Some(field) => match field.typ {
                _ => str::from_utf8(&self.data[field.start_index..field.start_index + field.len])
                    .unwrap_or("")
                    .to_string(),
            },
            _ => "".to_string(),
        }
    }
}

/// This struct represens a COBOL data
/// * data: contents of the COBOL data. The data field can be refered by other CobField.
/// * typ: the type of the COBOL data.
/// * digits: the number of digits. (e.g. digits of PIC 9(5) is 5) If the type of this COBOL data is numeric, the digits should be 0.
/// * scale: represents the scale specified by V and P of picture strings. (e.g. If the picture string is 9(3)V9(2), the scale is 2)
/// * flags: other options of a COBOL data. this option is an one of combinations of the following constants
///   * FLAG_NONE
///   * FLAG_HAVE_SIGN
///   * FLAG_SIGN_SEPARATE
///   * FLAG_SIGN_LEADING
///   * FLAG_BLANK_ZERO
///   * FLAG_JUSTIFIED
///   * FLAG_BINARY_SWAP
///   * FLAG_REAL_BINARY
///   * FLAG_IS_POINTER
/// * pic: the picture string of the COBOL data

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CobolField {
    pub start_index: usize,
    pub len: usize,
    pub typ: CobolFieldType,
    pub digits: usize,
    pub scale: i64,
    pub flags: u8,
    pic: String,
}

/// The COBOL field type
/// * Group: group field
/// * Bool: TODO
/// * NumericDisplay: PIC 9(5)
/// * Binary: PIC 9(5) COMP-5
/// * Packed: PIC 9(5) COMP-3
/// * Double: TODO
/// * NumericEdited: PIC 99,9, PIC 99/99
/// * Alphanumeric: PIC X(5)
/// * AlphanumericAll:
/// * AlphanumericEdited: PIC XX,X PIC XX/XX
/// * National: PIC N(3)
/// * NationalAll: TODO
/// * NationalEdited: PIC NN,N, PIC NN/NN
/// * Varying: PIC &, the extension of fukutsu COBOL
/// * Unicode: PIC U(8), the extension of fukutsu COBOL
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CobolFieldType {
    Group,
    Bool,
    NumericDisplay,
    Binary,
    Packed,
    Float,
    Double,
    NumericEdited,
    Alphanumeric,
    AlphanumericAll,
    AlphanumericEdited,
    National,
    NationalAll,
    NationalEdited,
    Varying,
    Unicode,
}

/// No option is specified.
pub const FLAG_NONE: u8 = 0x00;
/// The COBOL data is signed.
/// (e.g. PIC S9(5))
pub const FLAG_HAVE_SIGN: u8 = 0x1;
/// The sign symbol of COBOL data is signed.
/// (e.g. PIC 9(5) SIGN SEPARATE)
pub const FLAG_SIGN_SEPARATE: u8 = 0x2;
/// The sign symbol of COBOL data locates at the first byte.
/// (e.g. PIC 9(5) SIGN SEPARATE)
pub const FLAG_SIGN_LEADING: u8 = 0x4;
/// TODO
pub const FLAG_BLANK_ZERO: u8 = 0x8;
/// TODO
pub const FLAG_JUSTIFIED: u8 = 0x10;
/// TODO
pub const FLAG_BINARY_SWAP: u8 = 0x20;
/// TODO
pub const FLAG_REAL_BINARY: u8 = 0x40;
/// TODO
pub const FLAG_IS_POINTER: u8 = 0x80;

impl CobolField {
    /// Returns true if FLAG_HAVE_SIGN is specified. Otherwise returns false.
    pub fn is_have_sign(&self) -> bool {
        (self.flags & FLAG_HAVE_SIGN) != 0
    }

    /// Returns true if FLAG_SIGN_SEPARATE is specified. Otherwise returns false.
    pub fn is_sign_separate(&self) -> bool {
        (self.flags & FLAG_SIGN_SEPARATE) != 0
    }

    /// Returns true if FLAG_SIGN_SEPARATE is specified. Otherwise returns false.
    pub fn is_sign_leading(&self) -> bool {
        (self.flags & FLAG_BLANK_ZERO) != 0
    }

    /// Returns true if FLAG_SIGN_LEADING is specified. Otherwise returns false.
    pub fn is_blank_zero(&self) -> bool {
        (self.flags & FLAG_BLANK_ZERO) != 0
    }

    /// Returns true if FLAG_JUSTIFIED is specified. Otherwise returns false.
    pub fn is_justified(&self) -> bool {
        (self.flags & FLAG_JUSTIFIED) != 0
    }

    /// Returns true if FLAG_BINARY_SWAP is specified. Otherwise returns false.
    pub fn is_binary_swap(&self) -> bool {
        (self.flags & FLAG_BINARY_SWAP) != 0
    }

    /// Returns true if FLAG_REAL_BINARY is specified. Otherwise returns false.
    pub fn is_real_binary(&self) -> bool {
        (self.flags & FLAG_REAL_BINARY) != 0
    }

    /// Returns true if FLAG_IS_POINTER is specified. Otherwise returns false.
    pub fn is_pointer(&self) -> bool {
        (self.flags & FLAG_IS_POINTER) != 0
    }
}

type FieldId = usize;
