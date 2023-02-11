#[cfg(test)]
mod parser_test {
    lalrpop_mod!(pub parser);

    use crate::data::ast::*;

    #[test]
    fn parser_test() {
        assert_eq!(
            parser::CobolProgramParser::new().parse(
                r"
            identification division.
            program-id. hello.
            environment division.
            data division.
            procedure division."
            ),
            Ok(CobolProgram {
                identification_division: IdentificationDivision {
                    program_id: "hello"
                },
                environment_division: Some(EnvironmentDivision { dummy: "dummy" }),
                data_division: Some(DataDivision {
                    working_storage_section: None,
                }),
                procedure_division: Some(ProcedureDivision {
                    labelsStatements: Vec::new()
                }),
            })
        );
    }
}
