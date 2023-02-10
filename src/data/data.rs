use std::cmp::min;

pub struct CobData<'a> {
    pub data: &'a mut Vec<u8>,
    pub typ: CobFieldType,
    pub digits: usize,
    pub scale: i64,
    pub flags: u8,
    pub pic: &'static str,
}

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
}

pub const FLAG_NONE: u8 = 0x00;
pub const FLAG_HAVE_SIGN: u8 = 0x1;
pub const FLAG_SIGN_SEPARATE: u8 = 0x2;
pub const FLAG_SIGN_LEADING: u8 = 0x4;
pub const FLAG_BLANK_ZERO: u8 = 0x8;
pub const FLAG_JUSTIFIED: u8 = 0x10;
pub const FLAG_BINARY_SWAP: u8 = 0x20;
pub const FLAG_REAL_BINARY: u8 = 0x40;
pub const FLAG_IS_POINTER: u8 = 0x80;

impl<'a> CobData<'a> {
    pub fn is_have_sign(&self) -> bool {
        (self.flags & FLAG_HAVE_SIGN) != 0
    }
    pub fn is_sign_separate(&self) -> bool {
        (self.flags & FLAG_SIGN_SEPARATE) != 0
    }
    pub fn is_sign_leading(&self) -> bool {
        (self.flags & FLAG_BLANK_ZERO) != 0
    }
    pub fn is_blank_zero(&self) -> bool {
        (self.flags & FLAG_BLANK_ZERO) != 0
    }
    pub fn is_justified(&self) -> bool {
        (self.flags & FLAG_JUSTIFIED) != 0
    }
    pub fn is_binary_swap(&self) -> bool {
        (self.flags & FLAG_BINARY_SWAP) != 0
    }
    pub fn is_real_binary(&self) -> bool {
        (self.flags & FLAG_REAL_BINARY) != 0
    }
    pub fn is_pointer(&self) -> bool {
        (self.flags & FLAG_IS_POINTER) != 0
    }
    pub fn move_from(&mut self, src: &CobData) {
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
