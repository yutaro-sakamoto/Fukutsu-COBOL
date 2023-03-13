pub enum AbstractCode {
    Func(String, Vec<String>),
    // let <var_name> = <func_name>(<args>)
    LetVarFunc(String, String, Vec<String>),
}
