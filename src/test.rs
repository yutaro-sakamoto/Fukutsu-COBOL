#[cfg(test)]
mod parser_test {
    lalrpop_mod!(pub parser);

    use crate::data::ast::*;
    use std::collections::VecDeque;

    #[test]
    fn parser_test() {
        assert_eq!(
            parser::CobolProgramParser::new().parse(
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
            ),
            Ok(CobolProgram {
                identification_division: IdentificationDivision {
                    program_id: "hello"
                },
                environment_division: Some(EnvironmentDivision { dummy: "dummy" }),
                data_division: Some(DataDivision {
                    working_storage_section: Some(WorkingStorageSection {
                        data_descriptions: VecDeque::from(vec![
                            DataDescription {
                                level_number: 1,
                                entry_name: "ab",
                                description_clauses: vec![
                                    DataDescriptionClause::Picture(Picture::Alphanumeric {
                                        pic: "xx",
                                        len: 2
                                    }),
                                    DataDescriptionClause::Value("\"ab\"".to_string()),
                                ]
                            },
                            DataDescription {
                                level_number: 1,
                                entry_name: "cd",
                                description_clauses: vec![
                                    DataDescriptionClause::Picture(Picture::Alphanumeric {
                                        pic: "xx",
                                        len: 2
                                    }),
                                    DataDescriptionClause::Value("\"cd\"".to_string()),
                                ]
                            },
                        ])
                    }),
                }),
                procedure_division: Some(ProcedureDivision {
                    labels_statements: vec![
                        LabelStatement::Statement(Statement::Move(MoveStatement {
                            srcs: vec!["ab"],
                            dsts: vec!["cd"],
                        })),
                        LabelStatement::Statement(Statement::Display(DisplayStatement {
                            args: vec!["cd"]
                        }))
                    ]
                }),
            })
        );
    }
}
