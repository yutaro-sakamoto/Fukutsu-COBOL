use crate::abstract_code::{self, AbstractCode, AbstractExpr};

pub fn generate_code(abstract_code_list: &Vec<AbstractCode>) -> String {
    let header = r#"
const wasm = require("./pkg");
let core = wasm.CobolCore.new_by_string("hello_world");
"#
    .to_string();
    let lines: Vec<String> = abstract_code_list
        .iter()
        .map(|x| match x {
            AbstractCode::Expr(expr) => format!("{};", expr_to_string(expr)),
            AbstractCode::Let(var_name, expr) => {
                format!("let field_{} = {};", var_name, expr_to_string(expr))
            }
        })
        .collect();
    header + &lines.join("\n")
}

fn expr_to_string(expr: &AbstractExpr) -> String {
    match expr {
        AbstractExpr::FieldIdentifier(s) => format!("field_{}", s),
        AbstractExpr::Identifier(s) => s.to_string(),
        AbstractExpr::Int(i) => i.to_string(),
        AbstractExpr::UInt(u) => u.to_string(),
        // TODO escape ", \n, ... etc
        AbstractExpr::String(s) => format!("\"{}\"", s).to_string(),
        AbstractExpr::Func(name, args) => format!(
            "{} ({})",
            name,
            args.iter()
                .map(|arg| expr_to_string(arg))
                .collect::<Vec<String>>()
                .join(", ")
        ),
    }
}
