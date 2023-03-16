pub enum AbstractCode {
    Let(String, AbstractExpr),
    Expr(AbstractExpr),
}

pub enum AbstractExpr {
    Func(String, Vec<AbstractExpr>),
    String(String),
    Identifier(String),
    UInt(u32),
    Int(i32),
}
