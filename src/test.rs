#[cfg(test)]
mod parser_test {
    lalrpop_mod!(pub parser);

    use crate::data::ast::*;

    #[test]
    fn parser_test() {
        assert_eq!(
            parser::CobolProgramParser::new().parse(
                r#"
            identification division.
            program-id. hello.
            environment division.
            data division.
            working-storage section.
            01 ab pic xx value "ab".
            01 cd pic xx value "cd".
            procedure division."#
            ),
            Ok(CobolProgram {
                identification_division: IdentificationDivision {
                    program_id: "hello"
                },
                environment_division: Some(EnvironmentDivision { dummy: "dummy" }),
                data_division: Some(DataDivision {
                    working_storage_section: Some(WorkingStorageSection {
                        data_descriptions: vec![
                            DataDescription {
                                level_number: 1,
                                entry_name: "ab",
                                description_clauses: vec![
                                    DataDescriptionClause::Picture("xx"),
                                    DataDescriptionClause::Value("\"ab\""),
                                ]
                            },
                            DataDescription {
                                level_number: 1,
                                entry_name: "cd",
                                description_clauses: vec![
                                    DataDescriptionClause::Picture("xx"),
                                    DataDescriptionClause::Value("\"cd\""),
                                ]
                            },
                        ]
                    }),
                }),
                procedure_division: Some(ProcedureDivision {
                    labels_statements: Vec::new()
                }),
            })
        );
    }
}
