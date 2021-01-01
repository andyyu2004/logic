use crate::symbol::Symbol;

/// top level program
#[derive(Debug)]
pub struct ProgramClauses {
    clauses: Vec<Clause>,
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
    Implication(Term, Vec<Goal>),
    /// <clause>,<clause>
    And(Box<Clause>, Box<Clause>),
    // todo forall
}

#[derive(Debug)]
pub struct Atom(Symbol);

#[derive(Debug)]
pub struct Var(Symbol);

#[derive(Debug)]
pub enum Term {
    Var(Symbol),
    Compound(Atom, Vec<Term>),
}
