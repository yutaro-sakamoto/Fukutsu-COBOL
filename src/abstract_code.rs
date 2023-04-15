#[derive(Clone, Debug)]
pub enum AbstractCode<'a> {
    Let(&'a str, AbstractExpr<'a>),
    LetField(&'a str, AbstractExpr<'a>),
    Expr(AbstractExpr<'a>),
    GetNewCore(u32),
    LineComment(String),
}

#[derive(Clone, Debug)]
pub enum AbstractExpr<'a> {
    Func(&'a str, Vec<AbstractExpr<'a>>),
    LibFunc(&'a str, Vec<AbstractExpr<'a>>),
    LibCoreFunc(&'a str, Vec<AbstractExpr<'a>>),
    Str(&'a str),
    String(String),
    FieldIdentifier(&'a str),
    Identifier(&'a str),
    LibIdentifier(&'a str),
    UInt(u32),
    Int(i32),
    Bytes(Vec<u8>),
}
