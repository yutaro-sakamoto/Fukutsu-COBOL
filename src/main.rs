#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);
use data::ast::*;

mod abstract_code;
mod data;
mod gen_abstract_code;
mod gen_code;
mod test;
fn main() {
    let sample_source = r#"
    identification division.
    program-id. hello.
    environment division.
    DATA division.
    Working-storage section.
    01 ab PIC xx value "ab".
    01 cd pic xx value "cd".
    procedure division.
    move ab to cd.
    DisPlay cd.
    DisPlay ab."#;
    let ast = parser::CobolProgramParser::new()
        .parse(sample_source)
        .expect("[Error] parse error");
    let data_description_root_node = DataDescription {
        level_number: 0,
        entry_name: "#!@dummy@!#",
        description_clauses: Vec::new(),
    };
    let abstract_code =
        gen_abstract_code::generate_abstract_code(&ast, &data_description_root_node)
            .expect("[Error] code geenration error");
    let js_code = gen_code::js::generate_code(&abstract_code);
    println!("{}", js_code);
}
