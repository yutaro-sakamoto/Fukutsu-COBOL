pub struct CobData {
    pub size: usize,
    pub data: &[u8],
    pub attr: &mut CobFieldAttribute,
}

pub struct CobFieldAttribute {
    pub typ: CobFieldType,
    pub digits: usize,
    pub scale: i64,
    pub flags: CobFieldFlag,
    pub pic: &str,
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
    AlphphanumericEdited,
}

pub enum CobFieldFlag {
    HaveSign,
    SignSeparate,
    SignLeading,
    BlankZero,
    Justified,
    BinarySwap,
    RealBinary,
    IsPointer,
}
