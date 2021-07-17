#[derive(Debug)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Fact(Relation, Vec<Const>),
    Rule(Atom, Vec<Atom>),
}

#[derive(Debug)]
pub struct Fact {
    relation: Relation,
    args: Vec<Symbol>,
}

pub type Relation = Const;

#[derive(Debug)]
pub struct Const(pub Symbol);

#[derive(Debug)]
pub struct Var(pub Symbol);

#[derive(Debug)]
pub enum Term {
    Const(Const),
    Var(Var),
}

#[derive(Debug)]
pub struct Symbol(String);

impl Symbol {
    pub fn intern(s: &str) -> Self {
        Self(s.to_owned())
    }
}

#[derive(Debug)]
pub struct Rule {
    atom: Atom,
    terms: Vec<Term>,
}

#[derive(Debug)]
pub struct Atom {
    pub relation: Relation,
    pub terms: Vec<Term>,
}
