// https://www.youtube.com/watch?v=RwBiHLoQ3E4&ab_channel=PapersWeLove

mod ast_lowering;
mod debug;
mod interned;
mod interner;

pub mod tls;

pub use ast_lowering::{lower_ast, lower_goal};
pub use debug::DebugCtxt;
pub use interned::*;
pub use interner::Interner;
pub use logic_parse::{Atom, Sym, Var};
pub use std::ops::{Deref, DerefMut};

use indexed_vec::newtype_index;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

/// an interner that doesn't really intern anything
// the default "interner" for internal use
#[derive(Debug, Clone, Eq, PartialEq, Ord, Hash, PartialOrd, Copy)]
pub struct IRInterner;

impl Interner for IRInterner {
    // This and interned term TODO
    type ConcreteTerm = PrologTermData<Self>;
    type DomainGoal = SomeDomainGoal<Self, Self::ConcreteTerm>;
    // type DomainGoal = GenericTerm<Self>;
    // wrapped in `Rc` to make it cheaply cloneable
    // a proper interner should probably use copyable references
    type InternedClause = Rc<ClauseData<Self>>;
    type InternedClauses = Vec<Clause<Self>>;
    type InternedGoal = Rc<GoalData<Self>>;
    type InternedGoals = Vec<Goal<Self>>;
    type InternedTerm = Rc<Self::ConcreteTerm>;
    type InternedTerms = Vec<Term<Self>>;

    fn goal_data<'a>(&self, goal: &'a Self::InternedGoal) -> &'a GoalData<Self> {
        goal
    }

    fn goals<'a>(&self, goals: &'a Self::InternedGoals) -> &'a [Goal<Self>] {
        goals.as_slice()
    }

    fn intern_goal(self, goal: GoalData<Self>) -> Self::InternedGoal {
        Rc::new(goal)
    }

    fn intern_goals(self, goals: impl IntoIterator<Item = Goal<Self>>) -> Self::InternedGoals {
        goals.into_iter().collect()
    }

    fn clause_data<'a>(&self, clause: &'a Self::InternedClause) -> &'a ClauseData<Self> {
        clause
    }

    fn clauses<'a>(&self, clauses: &'a Self::InternedClauses) -> &'a [Clause<Self>] {
        clauses.as_slice()
    }

    fn intern_clause(self, clause: ClauseData<Self>) -> Self::InternedClause {
        Rc::new(clause)
    }

    fn intern_clauses(
        self,
        clauses: impl IntoIterator<Item = Clause<Self>>,
    ) -> Self::InternedClauses {
        clauses.into_iter().collect()
    }

    fn term_data<'a>(&self, term: &'a Self::InternedTerm) -> &'a Self::ConcreteTerm {
        todo!()
    }

    fn terms<'a>(&self, terms: &'a Self::InternedTerms) -> &'a [Term<Self>] {
        todo!()
    }

    fn intern_term(self, term: Self::ConcreteTerm) -> Self::InternedTerm {
        todo!()
    }

    fn intern_terms(self, term: impl IntoIterator<Item = Term<Self>>) -> Self::InternedTerms {
        todo!()
    }
}

/// top level program
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Program<I: Interner> {
    pub clauses: Clauses<I>,
    pub interner: I,
}

impl<I: Interner> Program<I> {
    pub fn new(interner: I, clauses: Clauses<I>) -> Self {
        Self { interner, clauses }
    }
}

// intuitively "things we want to prove"
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum GoalData<I: Interner> {
    DomainGoal(I::DomainGoal),
    And(Goal<I>, Goal<I>),
    Or(Goal<I>, Goal<I>),
    // todo exists, impl, forall
}

impl<I: Interner> Debug for GoalData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DomainGoal(domain_goal) => write!(f, "{:?}", domain_goal),
            _ => todo!(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ClauseData<I: Interner> {
    /// <clause> :- <goals>
    /// empty goal means the implication is a fact
    Horn(I::DomainGoal, Goals<I>),
    // todo forall
}

impl<I: Interner> Debug for ClauseData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ClauseData::Horn(consequent, conditions) =>
                if conditions.is_empty() {
                    write!(f, "{:?}", consequent)
                } else {
                    write!(f, "{:?} :- {:?}", consequent, conditions)
                },
        }
    }
}

// base type, generalization of what will be a `L Ty`
// maybe term and generic term not the best names
pub trait GenericTerm<I: Interner> {}

impl<I: Interner> GenericTerm<I> for Term<I> {
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Ty<I: Interner> {
    _marker: std::marker::PhantomData<I>,
}

impl<I: Interner> GenericTerm<I> for Ty<I> {
}

pub trait DomainGoal<I: Interner, T: GenericTerm<I>> {}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct SomeDomainGoal<I: Interner, T: GenericTerm<I>> {
    _marker_i: std::marker::PhantomData<I>,
    _marker_t: std::marker::PhantomData<T>,
}

impl<I: Interner, T: GenericTerm<I>> DomainGoal<I, T> for SomeDomainGoal<I, T> {
}

newtype_index!(InferenceIdx);

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PrologTermData<I: Interner> {
    Atom(Atom),
    Var(Var),
    Structure(Atom, Terms<I>),
    Infer(InferenceIdx),
}

impl<I> GenericTerm<I> for PrologTermData<I> where I: Interner
{
}

impl<I: Interner> Debug for PrologTermData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PrologTermData::Atom(atom) => write!(f, "{}", atom),
            PrologTermData::Var(var) => write!(f, "{}", var),
            PrologTermData::Structure(atom, terms) => write!(f, "{}({:?})", atom, terms),
            PrologTermData::Infer(infer) => write!(f, "{:?}", infer),
        }
    }
}
