pub enum AbstractCode {
    Func(String, Vec<AbstractPrimitive>),
    // let <var_name> = <func_name>(<args>)
    LetVarFunc(String, String, Vec<AbstractPrimitive>),
}

pub enum AbstractPrimitive {
    String(String),
    Identifier(String),
    UInt(u32),
    Int(i32),
}
