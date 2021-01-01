use crate::symbol::Symbol;

/// top level program
#[derive(Debug)]
pub struct ProgramClauses {
    clauses: Vec<Clause>,
}

impl ProgramClauses {
    pub fn new(clauses: Vec<Clause>) -> Self {
        Self { clauses }
    }
}

#[derive(Debug)]
pub enum Goal {
    Atom(Term),
    And(Box<Goal>, Box<Goal>),
    Or(Box<Goal>, Box<Goal>),
    // todo exists, impl, forall
}

#[derive(Debug)]
pub enum Clause {
    /// <clause> :- <goals>
    /// empty goal means the implication is a fact
    Horn(Term, Vec<Goal>),
    /// <clause>,<clause>
    And(Box<Clause>, Box<Clause>),
    // todo forall
}

#[derive(Debug)]
pub struct Var(Symbol);

impl Var {
    pub fn new(symbol: Symbol) -> Self {
        // variables must start with an uppercase
        debug_assert!(symbol.as_str().chars().next().unwrap().is_ascii_uppercase());
        Self(symbol)
    }
}

#[derive(Debug)]
pub struct Atom(Symbol);

impl Atom {
    pub fn new(symbol: Symbol) -> Self {
        // variables must start with an lowercase
        debug_assert!(symbol.as_str().chars().next().unwrap().is_ascii_lowercase());
        Self(symbol)
    }
}

/// a.k.a DomainGoal
#[derive(Debug)]
pub enum Term {
    Atom(Atom),
    Var(Var),
    Compound(Atom, Vec<Term>),
}
