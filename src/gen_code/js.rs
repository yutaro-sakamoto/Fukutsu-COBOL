use crate::abstract_code::{self, AbstractCode, AbstractPrimitive};

pub fn generate_code(abstract_code_list: &Vec<AbstractCode>) -> String {
    let lines: Vec<String> = abstract_code_list
        .iter()
        .map(|x| match x {
            AbstractCode::Func(func_name, args) => format!(
                "{} ({});",
                func_name,
                args.iter()
                    .map(|arg| primitive_to_string(arg))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            AbstractCode::LetVarFunc(var_name, func_name, args) => {
                format!(
                    "let {} = {} ({});",
                    var_name,
                    func_name,
                    args.iter()
                        .map(|arg| primitive_to_string(arg))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        })
        .collect();
    lines.join("\n")
}

fn primitive_to_string(primitive: &AbstractPrimitive) -> String {
    match primitive {
        AbstractPrimitive::Identifier(s) => s.to_string(),
        AbstractPrimitive::Int(i) => i.to_string(),
        AbstractPrimitive::UInt(u) => u.to_string(),
        // TODO escape ", \n, ... etc
        AbstractPrimitive::String(s) => format!("\"{}\"", s).to_string(),
    }
}
