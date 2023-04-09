use super::target::TranslateLanguage;
use crate::abstract_code::{self, AbstractCode, AbstractExpr};

pub fn generate_code(target: TranslateLanguage, abstract_code_list: &Vec<AbstractCode>) -> String {
    let header = match target {
        TranslateLanguage::NodeJS => {
            r#"const cb_lib = require("./fcbl_lib");
"#
        }
        _ => {
            r#"import * as cb_lib from "fukutsu-cobol";

export function cb_run() {
"#
        }
    }
    .to_string();
    let lines: Vec<String> = abstract_code_list
        .iter()
        .map(|x| match x {
            AbstractCode::Expr(expr) => format!("{};", expr_to_string(expr)),
            AbstractCode::Let(var_name, expr) => {
                format!("let {} = {};", var_name, expr_to_string(expr))
            }
            AbstractCode::LetField(var_name, expr) => {
                format!("let field_{} = {};", var_name, expr_to_string(expr))
            }
            AbstractCode::GetNewCore(size) => {
                format!("let cb_core = cb_lib.get_fcbl_core({});", size)
            }
        })
        .collect();
    match target {
        TranslateLanguage::NodeJS => header + &lines.join("\n"),
        _ => {
            header
                + &lines
                    .iter()
                    .map(|line| format!("  {}", line))
                    .collect::<Vec<String>>()
                    .join("\n")
                + "\n}\n"
        }
    }
}

fn expr_to_string(expr: &AbstractExpr) -> String {
    match expr {
        AbstractExpr::FieldIdentifier(s) => format!("field_{}", s),
        AbstractExpr::Identifier(s) => s.to_string(),
        AbstractExpr::LibIdentifier(s) => format!("cb_lib.{}", s.to_string()),
        AbstractExpr::Int(i) => i.to_string(),
        AbstractExpr::UInt(u) => u.to_string(),
        // TODO escape ", \n, ... etc
        AbstractExpr::String(s) => format!("\"{}\"", s).to_string(),
        AbstractExpr::Str(s) => format!("\"{}\"", s).to_string(),
        AbstractExpr::Bytes(bytes) => format!(
            "new Uint8Array([{}])",
            bytes
                .iter()
                .map(|b| format!("{}", b))
                .collect::<Vec<String>>()
                .join(", ")
        ),
        AbstractExpr::Func(name, args) => format!(
            "{} ({})",
            name,
            args.iter()
                .map(|arg| expr_to_string(arg))
                .collect::<Vec<String>>()
                .join(", ")
        ),
        AbstractExpr::LibFunc(name, args) => format!(
            "cb_lib.{} ({})",
            name,
            args.iter()
                .map(|arg| expr_to_string(arg))
                .collect::<Vec<String>>()
                .join(", ")
        ),
        AbstractExpr::LibCoreFunc(name, args) => format!(
            "cb_lib.{} (cb_core, {})",
            name,
            args.iter()
                .map(|arg| expr_to_string(arg))
                .collect::<Vec<String>>()
                .join(", ")
        ),
    }
}
