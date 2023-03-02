use super::abstract_code::AbstractCode;
use super::data::ast::*;

pub fn generate_abstract_code<'a>(program: &'a CobolProgram) -> Vec<AbstractCode> {
    let procedure_division_code = match &program.procedure_division {
        Some(procedure_division) => procedure_division
            .labels_statements
            .iter()
            .map(|x| match x {
                LabelStatement::Section(name) => {
                    vec![AbstractCode::Func(
                        "console.log".to_string(),
                        vec![format!("'Section: {}'", name)],
                    )]
                }
                LabelStatement::Label(name) => {
                    vec![AbstractCode::Func(
                        "console.log".to_string(),
                        vec![format!("'Label: {}'", name)],
                    )]
                }
                LabelStatement::Statement(Statement::Move(st)) => convert_move_statement(st),
                LabelStatement::Statement(Statement::Display(st)) => convert_display_statement(st),
            })
            .into_iter()
            .flatten()
            .collect(),
        None => Vec::new(),
    };
    procedure_division_code
}

fn convert_move_statement<'a>(st: &MoveStatement<'a>) -> Vec<AbstractCode> {
    if st.srcs.len() == 1 {
        let src_name = format!("{}_field", st.srcs[0]);
        st.dsts
            .iter()
            .map(|dst| {
                AbstractCode::Func(
                    "core.move_field".to_string(),
                    vec![src_name.clone(), format!("{}_field", dst)],
                )
            })
            .collect()
    } else {
        st.srcs
            .iter()
            .zip(st.dsts.iter())
            .map(|(src, dst)| {
                AbstractCode::Func(
                    "core.move_field".to_string(),
                    vec![format!("{}_field", src), format!("{}_field", dst)],
                )
            })
            .collect()
    }
}

fn convert_display_statement<'a>(st: &DisplayStatement<'a>) -> Vec<AbstractCode> {
    let args: Vec<String> = st.args.iter().map(|arg| format!("'{}'", arg)).collect();
    vec![AbstractCode::Func(
        "console.log".to_string(),
        vec![args.join(" + ")],
    )]
}
