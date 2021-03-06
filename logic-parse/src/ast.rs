use crate::symbol::Symbol;
use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};

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
    DomainGoal(DomainGoal),
    And(Box<Goal>, Box<Goal>),
    Or(Box<Goal>, Box<Goal>),
    Implies(Box<Clause>, Box<Goal>),
    Exists(Vec<Var>, Box<Goal>), // Quantified(Quantifier, Binders, Goal),,
                                 // todo exists, impl, forall, implies
}

impl Display for Goal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Goal::DomainGoal(domain_goal) => write!(f, "{}", domain_goal),
            Goal::And(lhs, rhs) => write!(f, "{} & {}", lhs, rhs),
            Goal::Or(lhs, rhs) => write!(f, "{} | {}", lhs, rhs),
            Goal::Implies(clause, goal) => write!(f, "{} => {}", clause, goal),
            Goal::Exists(vars, goal) => write!(f, "∃{}.{}", util::join(vars, ","), goal),
        }
    }
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum DomainGoal {
    Holds(Constraint),
}

impl Display for DomainGoal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DomainGoal::Holds(constraint) => todo!(),
        }
    }
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Constraint {
    Implemented(ImplConstraint),
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Var {
    pub ident: Ident,
}

impl Var {
    pub fn new(ident: Ident) -> Self {
        Self { ident }
    }
}

// does `ty` implement `trait`?
#[derive(Debug, Eq, Clone, PartialEq)]
pub struct ImplConstraint {
    pub ty: Ty,
    pub trait_ref: TraitRef,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct TraitRef {
    pub trait_name: Ident,
    pub args: Vec<Ty>,
}

// "things we know"
#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Clause {
    DomainGoal(DomainGoal),
    Implies(Implication),
    // if we can prove goal, then clause is true,
    // *NOTE* the lhs of implication is a clause in the chalk grammar,
    // do we lose any expressiveness by making it a domaingoal?
    // <clause>,<clause>
    And(Box<Clause>, Box<Clause>),
}

impl Display for Clause {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Clause::DomainGoal(domain_goal) => write!(f, "{}", domain_goal),
            Clause::Implies(implication) => write!(f, "{}", implication),
            Clause::And(lhs, rhs) => write!(f, "{} && {}", lhs, rhs),
        }
    }
}

/// "forall" "<" <vars> ">" "{" <consequent> :- <condition> "}"
#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Implication {
    pub vars: Vec<Var>,
    pub consequent: DomainGoal,
    pub condition: Goal,
}

impl Display for Implication {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "for<{}> {{ {} :- {} }}",
            util::join(&self.vars, ","),
            self.consequent,
            self.condition
        )
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Ident {
    pub span: Span,
    pub symbol: Symbol,
}

impl Ident {
    pub fn unspanned(s: &str) -> Self {
        Self { span: Span::new(0, 0), symbol: Symbol::from(s) }
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state)
    }
}

#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub fn new(lo: usize, hi: usize) -> Self {
        assert!(lo <= hi);
        Self { lo, hi }
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl Ident {
    pub fn new(span: Span, symbol: Symbol) -> Self {
        Self { span, symbol }
    }
}

/// a.k.a DomainGoal
#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Ty {
    Structure(Ident, Vec<Ty>),
}

impl Display for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Structure(functor, terms) => write!(f, "{}<{}>", functor, util::join(terms, ",")),
        }
    }
}
