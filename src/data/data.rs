use std::cmp::min;

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
pub struct CobField<'a> {
    pub data: &'a mut Vec<u8>,
    pub typ: CobFieldType,
    pub digits: usize,
    pub scale: i64,
    pub flags: u8,
    pub pic: &'static str,
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
pub enum CobFieldType {
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

impl<'a> CobField<'a> {
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

    /// The process of MOVE statement.
    /// This function write data into from `src`.
    pub fn move_from(&mut self, src: &CobField) {
        match self.typ {
            CobFieldType::Alphanumeric => match src.typ {
                CobFieldType::Alphanumeric
                | CobFieldType::AlphanumericAll
                | CobFieldType::AlphanumericEdited
                | CobFieldType::National
                | CobFieldType::NationalAll
                | CobFieldType::NationalEdited => {
                    let m = min(self.data.len(), src.data.len());
                    for i in 0..m {
                        self.data[i] = src.data[i];
                    }
                    for i in m..self.data.len() {
                        self.data[i] = ' ' as u8;
                    }
                }
                _ => unreachable!("The unimplemented case of move From"),
            },
            _ => unreachable!("The unimplemented case of move From"),
        }
    }
}
