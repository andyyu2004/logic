#[test]
fn parse_datalog_program() {
    let src = r#"
        fact mortal(socrates).

        mortal(X) :- man(X).
    "#;
    let parser = crate::datalog::ProgramParser::new();
    parser.parse(src).unwrap();
}

#[test]
fn parse_fact_with_variable() {
    let src = r#"
        fact mortal(X).
    "#;
    let parser = crate::datalog::ProgramParser::new();
    parser.parse(src).unwrap();
}
