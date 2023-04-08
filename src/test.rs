#[cfg(test)]
mod parser_test {
    lalrpop_mod!(pub parser);

    use crate::data::ast::*;
    use std::collections::VecDeque;

    #[test]
    fn parser_test() {
        assert!(parser::CobolProgramParser::new()
            .parse(
                r#"
            identification division.
            program-id. hello.
            environment division.
            DATA division.
            Working-storage section.
            01 ab PIC xx value "ab".
            01 cd pic xx value "cd".
            procedure division.
            move ab to cd.
            DisPlay cd."#
            )
            .is_ok());
    }
}
