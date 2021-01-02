pub mod ast;
mod parser;
mod symbol;

pub use ast::{Atom, Var};
pub use symbol::Symbol;

pub fn parse<'a, T, E: std::fmt::Display>(
    src: &'a str,
    parser: impl FnOnce(&'a str) -> Result<T, E>,
) -> Option<T> {
    match parser(src) {
        Ok(clause) => Some(clause),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}

pub fn parse_clause(src: &str) -> Option<ast::Clause> {
    parse(src, |src| parser::ClauseParser::new().parse(src))
}

pub fn parse_goal(src: &str) -> Option<ast::Goal> {
    parse(src, |src| parser::GoalParser::new().parse(src))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_term_test() {
        let _term = parser::TermParser::new().parse("a(b,c,d)").unwrap();
        let _term = parser::TermParser::new().parse("cool(bob)").unwrap();
    }

    #[test]
    fn parse_clause_test() {
        let _fact = parser::ClauseParser::new().parse("cool(bob)").unwrap();
        let _fact = parser::ClauseParser::new().parse("cool(?X)").unwrap();
        let _clause = parser::ClauseParser::new().parse("cool(bob) :- cool(frank)").unwrap();
        let _clause = parser::ClauseParser::new().parse("cool(bob) :- cool(f), cool(jen)").unwrap();
    }

    #[test]
    fn parse_program_test() {
        let _prog = parser::ProgramParser::new()
            .parse("cool(jen). cool(bob). cool(X) :- cool(jen).")
            .unwrap();
    }
}
