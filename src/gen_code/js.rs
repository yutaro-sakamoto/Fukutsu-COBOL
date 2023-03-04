use crate::abstract_code::{self, AbstractCode};

pub fn generate_code(abstract_code_list: &Vec<AbstractCode>) -> String {
    let lines: Vec<String> = abstract_code_list
        .iter()
        .map(|x| match x {
            AbstractCode::Func(func_name, args) => format!("{} ({});", func_name, args.join(", ")),
        })
        .collect();
    lines.join("\n")
}
