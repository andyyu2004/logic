mod ast;
mod parse_tests;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub datalog);

fn main() {
    println!("Hello, world!");
}
