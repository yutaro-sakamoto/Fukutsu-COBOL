#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);
use data::ast::*;

mod abstract_code;
mod data;
mod gen_abstract_code;
mod gen_code;
mod test;

use std::fs::File;
use std::io::prelude::*;

use rustop::opts;

fn print_version() {
    println!("Fukutsu-COBOL version 0.1.0");
    println!("Copyright (C) 2023 Yutaro Sakamoto");
}

fn main() -> std::io::Result<()> {
    let (args, _) = opts! {
        opt version:bool, desc: "Display the version of Fukutsu-COBOL";
        opt target:Option<String>, desc: "Specify the language conversion. [possible values: nodejs(default), web]";
        param infile:Option<String>, desc:"Input file name.";
        param outfile:Option<String>, desc:"Output file name.";
    }
    .parse_or_exit();

    if (args.version) {
        print_version();
        return Ok(());
    }

    match args.infile {
        None => {
            print_version();
            Ok(())
        }
        Some(ref infile) => {
            let source = std::fs::read_to_string(infile)
                .expect(format!("[Error] input file `{}` not found", infile).as_str());
            let ast = parser::CobolProgramParser::new()
                .parse(source.as_str())
                .expect("[Error] parse error");
            let data_description_root_node = DataDescription {
                level_number: 0,
                entry_name: "#!@dummy@!#",
                description_clauses: Vec::new(),
            };
            let abstract_code =
                gen_abstract_code::generate_abstract_code(&ast, &data_description_root_node)
                    .expect("[Error] code geenration error");

            let js_code = match args.target {
                Some(s) => {
                    if s.eq("web") {
                        gen_code::js::generate_code(
                            gen_code::target::TranslateLanguage::WebJS,
                            &abstract_code,
                        )
                    } else if s.eq("nodejs") {
                        gen_code::js::generate_code(
                            gen_code::target::TranslateLanguage::NodeJS,
                            &abstract_code,
                        )
                    } else {
                        eprintln!("Error: Unknown target `{}`", s);
                        return Ok(());
                    }
                }
                None => gen_code::js::generate_code(
                    gen_code::target::TranslateLanguage::NodeJS,
                    &abstract_code,
                ),
            };

            let file_path = match args.outfile {
                Some(file) => file,
                None => format!("{}.js", ast.identification_division.program_id.to_string()),
            };

            let mut file = File::create(file_path)?;
            file.write_all(js_code.as_bytes())?;

            Ok(())
        }
    }
}
