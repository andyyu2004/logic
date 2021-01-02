use crate::symbol::Symbol;
use std::fmt::{self, Display, Formatter};

/// top level program
#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for item in &self.items {
            writeln!(f, "{}.", item)?;
        }
        Ok(())
    }
}

impl Program {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Item {
    Clause(Clause),
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Item::Clause(clause) => write!(f, "{}", clause),
        }
    }
}

#[derive(Debug, Eq, Clone, PartialEq)]
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

#[derive(Debug, Eq, Clone, PartialEq)]
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
#[derive(Debug, Eq, Clone, PartialEq)]
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
