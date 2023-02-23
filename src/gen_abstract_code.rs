use super::abstract_code::AbstractCode;
use super::data::ast::*;

pub fn generate_abstract_code<'a>(program: &'a CobolProgram) -> Vec<AbstractCode<'a>> {
    vec![
        AbstractCode::Func("console.log", vec!["'hello'"]),
        AbstractCode::Func("console.log", vec!["'world'"]),
    ]
}
