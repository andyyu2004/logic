use crate::symbol::Symbol;
use std::fmt::{self, Display, Formatter};

/// top level program
#[derive(Debug)]
pub struct ProgramClauses {
    clauses: Vec<Clause>,
}

impl Display for ProgramClauses {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for clause in &self.clauses {
            writeln!(f, "{}.", clause)?;
        }
        Ok(())
    }
}

impl ProgramClauses {
    pub fn new(clauses: Vec<Clause>) -> Self {
        Self { clauses }
    }
}

#[derive(Debug)]
pub enum Goal {
    Term(Term),
    And(Box<Goal>, Box<Goal>),
    Or(Box<Goal>, Box<Goal>),
    // todo exists, impl, forall
}

impl Display for Goal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Goal::Term(term) => write!(f, "{}", term),
            Goal::And(lhs, rhs) => write!(f, "{} & {}", lhs, rhs),
            Goal::Or(lhs, rhs) => write!(f, "{} | {}", lhs, rhs),
        }
    }
}

#[derive(Debug)]
pub enum Clause {
    /// <domain-goal> :- <goals>
    /// empty goal means the implication is a fact
    Horn(Term, Vec<Goal>),
    // <clause>,<clause>
    // And(Box<Clause>, Box<Clause>),
    // todo forall
}

impl Display for Clause {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Clause::Horn(term, goals) =>
                if goals.is_empty() {
                    write!(f, "{}", term)
                } else {
                    write!(f, "{} :- {}", term, util::join(goals, ", "))
                },
            // Clause::And(lhs, rhs) => write!(f, "{} & {}", lhs, rhs),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Var(Symbol);

impl Var {
    pub fn new(symbol: Symbol) -> Self {
        assert!(symbol.as_str().chars().next().unwrap() == '?');
        Self(symbol)
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Atom(Symbol);

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Atom {
    pub fn new(symbol: Symbol) -> Self {
        Self(symbol)
    }
}

/// a.k.a DomainGoal
#[derive(Debug)]
pub enum Term {
    Atom(Atom),
    Var(Var),
    Structure(Atom, Vec<Term>),
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Term::Atom(atom) => write!(f, "{}", atom),
            Term::Var(var) => write!(f, "{}", var),
            Term::Structure(atom, terms) => write!(f, "{}({})", atom, util::join(terms, ",")),
        }
    }
}
