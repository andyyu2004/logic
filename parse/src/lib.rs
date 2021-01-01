mod ast;
mod parser;
mod symbol;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_term() {
        let _term = parser::TermParser::new().parse("a(b,c,d)").unwrap();
        let _term = parser::TermParser::new().parse("cool(bob)").unwrap();
    }

    #[test]
    fn parse_clause() {
        let _fact = parser::ClauseParser::new().parse("cool(bob)").unwrap();
        let _clause = parser::ClauseParser::new().parse("cool(bob) :- cool(frank)").unwrap();
        let _clause = parser::ClauseParser::new().parse("cool(bob) :- cool(f), cool(jen)").unwrap();
    }
}
