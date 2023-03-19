#[derive(Clone, Debug)]
pub enum AbstractCode<'a> {
    Let(&'a str, AbstractExpr<'a>),
    Expr(AbstractExpr<'a>),
}

#[derive(Clone, Debug)]
pub enum AbstractExpr<'a> {
    Func(&'a str, Vec<AbstractExpr<'a>>),
    String(&'a str),
    FieldIdentifier(&'a str),
    Identifier(&'a str),
    UInt(u32),
    Int(i32),
}
