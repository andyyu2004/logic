mod ast;
mod parser;
mod symbol;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_term() {
        let term = parser::TermParser::new().parse("a(b,c,d)").unwrap();
        dbg!(term);
        let term = parser::TermParser::new().parse("cool(bob)").unwrap();
        dbg!(term);
    }

    #[test]
    fn parse_clause() {
        // let fact = parser::ClauseParser::new().parse("cool(bob)").unwrap();
        // dbg!(fact);
        // let clause = parser::ClauseParser::new().parse("cool(bob) :- cool(frank)").unwrap();
        // dbg!(clause);
    }
}
