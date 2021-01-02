// https://www.youtube.com/watch?v=RwBiHLoQ3E4&ab_channel=PapersWeLove

mod ast_lowering;
mod interner;

pub use interner::Interner;

use parse::{Atom, Var};
use std::fmt::{self, Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Ord, Hash, PartialOrd, Copy)]
struct IRInterner;

impl Interner for IRInterner {
    type InternedClause = Rc<Clause<Self>>;
    type InternedClauses = Vec<Clause<Self>>;
    type InternedGoal = Rc<Goal<Self>>;
    type InternedGoals = Vec<InternedGoal<Self>>;
    type InternedTerm = Rc<Term<Self>>;
    type InternedTerms = Vec<InternedTerm<Self>>;

    fn intern_goal(&self, goal: Goal<Self>) -> Self::InternedGoal {
        todo!()
    }

    fn intern_clause(&self, clause: Clause<Self>) -> Self::InternedClause {
        todo!()
    }

    fn intern_clauses(
        &self,
        clause: impl IntoIterator<Item = Clause<Self>>,
    ) -> Self::InternedClause {
        todo!()
    }

    fn intern_term(&self, term: Term<Self>) -> Self::InternedTerm {
        todo!()
    }

    fn intern_goals(
        &self,
        goals: impl IntoIterator<Item = InternedGoal<Self>>,
    ) -> Self::InternedGoals {
        todo!()
    }

    fn intern_terms(
        &self,
        terms: impl IntoIterator<Item = InternedTerm<Self>>,
    ) -> Self::InternedTerms {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Interned<T> {
    interned: T,
}

impl<T> Interned<T> {
    pub fn new(interned: T) -> Self {
        Self { interned }
    }
}

pub type InternedClause<I> = Interned<<I as Interner>::InternedClause>;
pub type InternedGoal<I> = Interned<<I as Interner>::InternedGoal>;
pub type InternedGoals<I> = Interned<<I as Interner>::InternedGoals>;
pub type InternedTerm<I> = Interned<<I as Interner>::InternedTerm>;
pub type InternedTerms<I> = Interned<<I as Interner>::InternedTerms>;

/// top level program
#[derive(Debug)]
pub struct ProgramClauses<I: Interner> {
    clauses: Vec<Clause<I>>,
}

impl<I: Interner> ProgramClauses<I> {
    pub fn new(clauses: Vec<Clause<I>>) -> Self {
        Self { clauses }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Goal<I: Interner> {
    Term(InternedTerm<I>),
    And(InternedGoal<I>, InternedGoal<I>),
    Or(InternedGoal<I>, InternedGoal<I>),
    // todo exists, impl, forall
}

impl<I: Interner> Debug for Goal<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Clause<I: Interner> {
    /// <clause> :- <goals>
    /// empty goal means the implication is a fact
    Horn(InternedTerm<I>, InternedGoals<I>),
    // todo forall
}

impl<I: Interner> Debug for Clause<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// a.k.a DomainGoal
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Term<I: Interner> {
    Atom(Atom),
    Var(Var),
    Structure(Atom, InternedTerms<I>),
}
