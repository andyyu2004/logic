// https://www.youtube.com/watch?v=RwBiHLoQ3E4&ab_channel=PapersWeLove

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
    /// <clause> :- <goal>
    Implication(Box<Clause>, Goal),
    /// <clause>,<clause>
    And(Box<Clause>, Box<Clause>),
    // todo forall
}

#[derive(Debug)]
pub struct Atom(Symbol);

#[derive(Debug)]
pub struct Var(Symbol);

#[derive(Debug)]
pub struct Symbol(usize);

#[derive(Debug)]
pub enum Term {
    Var(Symbol),
    Compound(Atom, Vec<Term>),
}
