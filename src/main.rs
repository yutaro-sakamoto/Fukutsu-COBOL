#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

mod data;
mod test;
fn main() {
    println!("Hello, world!");
}
