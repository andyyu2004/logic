// https://www.youtube.com/watch?v=RwBiHLoQ3E4&ab_channel=PapersWeLove

mod ast_lowering;
mod interner;

pub use ast_lowering::lower_ast;
pub use interner::Interner;

use parse::{Atom, Var};
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

/// an interner that doesn't really intern anything
#[derive(Debug, Clone, Eq, PartialEq, Ord, Hash, PartialOrd, Copy)]
pub struct IRInterner;

impl Interner for IRInterner {
    type InternedClause = Rc<Clause<Self>>;
    type InternedClauses = Vec<InternedClause<Self>>;
    type InternedGoal = Rc<Goal<Self>>;
    type InternedGoals = Vec<InternedGoal<Self>>;
    type InternedTerm = Rc<Term<Self>>;
    type InternedTerms = Vec<InternedTerm<Self>>;

    fn intern_goal(&self, goal: Goal<Self>) -> Self::InternedGoal {
        Rc::new(goal)
    }

    fn intern_clause(&self, clause: Clause<Self>) -> Self::InternedClause {
        Rc::new(clause)
    }

    fn intern_clauses(
        &self,
        clauses: impl IntoIterator<Item = InternedClause<Self>>,
    ) -> Self::InternedClauses {
        clauses.into_iter().collect()
    }

    fn intern_term(&self, term: Term<Self>) -> Self::InternedTerm {
        Rc::new(term)
    }

    fn intern_goals(
        &self,
        goals: impl IntoIterator<Item = InternedGoal<Self>>,
    ) -> Self::InternedGoals {
        goals.into_iter().collect()
    }

    fn intern_terms(
        &self,
        terms: impl IntoIterator<Item = InternedTerm<Self>>,
    ) -> Self::InternedTerms {
        terms.into_iter().collect()
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

pub type InternedClause<I> = <I as Interner>::InternedClause;
pub type InternedClauses<I> = <I as Interner>::InternedClauses;
pub type InternedGoal<I> = <I as Interner>::InternedGoal;
pub type InternedGoals<I> = <I as Interner>::InternedGoals;
pub type InternedTerm<I> = <I as Interner>::InternedTerm;
pub type InternedTerms<I> = <I as Interner>::InternedTerms;

/// top level program
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Program<I: Interner> {
    clauses: InternedClauses<I>,
}

impl<I: Interner> Program<I> {
    pub fn new(clauses: InternedClauses<I>) -> Self {
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
