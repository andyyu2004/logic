// https://www.youtube.com/watch?v=RwBiHLoQ3E4&ab_channel=PapersWeLove

mod ast_lowering;
mod debug;
mod interner;
pub mod tls;

pub use ast_lowering::lower_ast;
pub use debug::DebugCtxt;
pub use interner::Interner;
pub use std::ops::{Deref, DerefMut};

use parse::{Atom, Var};
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

/// an interner that doesn't really intern anything
#[derive(Debug, Clone, Eq, PartialEq, Ord, Hash, PartialOrd, Copy)]
pub struct IRInterner;

impl Interner for IRInterner {
    type InternedClause = Rc<ClauseData<Self>>;
    type InternedClauses = Vec<Clause<Self>>;
    type InternedGoal = Rc<GoalData<Self>>;
    type InternedGoals = Vec<Goal<Self>>;
    type InternedSubsts = Vec<Term<Self>>;
    type InternedTerm = Rc<TermData<Self>>;
    type InternedTerms = Vec<Term<Self>>;

    fn clause<'a>(&self, clause: &'a Self::InternedClause) -> &'a ClauseData<Self> {
        clause
    }

    fn clauses<'a>(&self, clauses: &'a Self::InternedClauses) -> &'a [Clause<Self>] {
        clauses.as_slice()
    }

    fn goal<'a>(&self, goal: &'a Self::InternedGoal) -> &'a GoalData<Self> {
        goal
    }

    fn goals<'a>(&self, goals: &'a Self::InternedGoals) -> &'a [Goal<Self>] {
        goals.as_slice()
    }

    fn term<'a>(&self, term: &'a Self::InternedTerm) -> &'a TermData<Self> {
        term
    }

    fn terms<'a>(&self, terms: &'a Self::InternedTerms) -> &'a [Term<Self>] {
        terms.as_slice()
    }

    fn intern_goal(self, goal: GoalData<Self>) -> Self::InternedGoal {
        Rc::new(goal)
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

    fn intern_term(self, term: TermData<Self>) -> Self::InternedTerm {
        Rc::new(term)
    }

    fn intern_goals(self, goals: impl IntoIterator<Item = Goal<Self>>) -> Self::InternedGoals {
        goals.into_iter().collect()
    }

    fn intern_terms(self, terms: impl IntoIterator<Item = Term<Self>>) -> Self::InternedTerms {
        terms.into_iter().collect()
    }

    fn intern_substs(self, substs: impl IntoIterator<Item = Term<Self>>) -> Self::InternedSubsts {
        substs.into_iter().collect()
    }
}

macro_rules! interned {
    ($ty:ident, $interned:ident) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $ty<I: Interner>(I::$interned);

        impl<I: Interner> Deref for $ty<I> {
            type Target = I::$interned;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<I: Interner> DerefMut for $ty<I> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

interned!(Goal, InternedGoal);
interned!(Goals, InternedGoals);
interned!(Clause, InternedClause);
interned!(Clauses, InternedClauses);
interned!(Term, InternedTerm);
interned!(Terms, InternedTerms);
interned!(Substs, InternedTerms);

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
    Term(Term<I>),
    And(Goal<I>, Goal<I>),
    Or(Goal<I>, Goal<I>),
    // todo exists, impl, forall
}

impl<I: Interner> Debug for GoalData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Term(term) => write!(f, "{:?}", term),
            _ => todo!(),
        }
    }
}

// intuitively "things we know"
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ClauseData<I: Interner> {
    /// <clause> :- <goals>
    /// empty goal means the implication is a fact
    Horn(Term<I>, Goals<I>),
    // todo forall
}

impl<I: Interner> Debug for ClauseData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ClauseData::Horn(consequent, conditions) =>
                write!(f, "{:?} :- {:?}", consequent, conditions),
        }
    }
}

/// a.k.a DomainGoal
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum TermData<I: Interner> {
    Atom(Atom),
    Var(Var),
    Structure(Atom, Terms<I>),
}

impl<I: Interner> Debug for TermData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TermData::Atom(atom) => writeln!(f, "{}", atom),
            TermData::Var(var) => writeln!(f, "{}", var),
            TermData::Structure(atom, terms) => writeln!(f, "{}({:?})", atom, terms),
        }
    }
}
