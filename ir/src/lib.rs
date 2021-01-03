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
// the default "interner" for internal use
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

    fn clause_data<'a>(&self, clause: &'a Self::InternedClause) -> &'a ClauseData<Self> {
        clause
    }

    fn clauses<'a>(&self, clauses: &'a Self::InternedClauses) -> &'a [Clause<Self>] {
        clauses.as_slice()
    }

    fn goal_data<'a>(&self, goal: &'a Self::InternedGoal) -> &'a GoalData<Self> {
        goal
    }

    fn goals<'a>(&self, goals: &'a Self::InternedGoals) -> &'a [Goal<Self>] {
        goals.as_slice()
    }

    fn term_data<'a>(&self, term: &'a Self::InternedTerm) -> &'a TermData<Self> {
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
    ($data:ident => $intern:ident => $ty:ident, $interned:ident, $dbg_method:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $ty<I: Interner> {
            interner: I,
            interned: I::$interned,
        }

        impl<I: Interner> $ty<I> {
            pub fn new(interner: I, interned: I::$interned) -> Self {
                Self { interner, interned }
            }

            pub fn intern(interner: I, data: $data<I>) -> Self {
                Self { interner, interned: interner.$intern(data) }
            }
        }

        impl<I: Interner> std::ops::Deref for $ty<I> {
            type Target = I::$interned;

            fn deref(&self) -> &Self::Target {
                &self.interned
            }
        }

        impl<I: Interner> std::ops::DerefMut for $ty<I> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.interned
            }
        }

        impl<I: Interner> Debug for $ty<I> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                self.interner.$dbg_method(self, f)
            }
        }
    };
}

macro_rules! interned_slice {
    ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident, $dbg_method:ident) => {
        /// List of interned elements.
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $seq<I: Interner> {
            interner: I,
            interned: I::$interned,
        }

        impl<I: Interner> $seq<I> {
            pub fn new(interner: I, iter: impl IntoIterator<Item = $elem>) -> Self {
                Self { interner, interned: interner.$intern(iter) }
            }

            pub fn interned(&self) -> &I::$interned {
                &self.interned
            }

            pub fn as_slice(&self, interner: &I) -> &[$elem] {
                Interner::$data(interner, &self.interned)
            }

            pub fn at(&self, interner: &I, index: usize) -> &$elem {
                &self.as_slice(interner)[index]
            }

            pub fn is_empty(&self, interner: &I) -> bool {
                self.as_slice(interner).is_empty()
            }

            pub fn iter(&self, interner: &I) -> std::slice::Iter<'_, $elem> {
                self.as_slice(interner).iter()
            }

            pub fn len(&self, interner: &I) -> usize {
                self.as_slice(interner).len()
            }
        }

        impl<I: Interner> std::ops::Deref for $seq<I> {
            type Target = I::$interned;

            fn deref(&self) -> &Self::Target {
                &self.interned
            }
        }

        impl<I: Interner> std::ops::DerefMut for $seq<I> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.interned
            }
        }

        impl<I: Interner> std::fmt::Debug for $seq<I> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                todo!()
            }
        }
    };
}

interned!(GoalData => intern_goal => Goal, InternedGoal, dbg_goal);
interned!(ClauseData => intern_clause => Clause, InternedClause, dbg_clause);
interned!(TermData => intern_term => Term, InternedTerm, dbg_term);

interned_slice!(
    Clauses, 
    clauses => Clause<I>, 
    intern_clauses => InternedClauses, 
    dbg_clauses);

interned_slice!(
    Goals, 
    goals => Goal<I>, 
    intern_goals => InternedGoals, 
    dbg_goals);

interned_slice!(
    Terms, 
    terms => Term<I>, 
    intern_terms => InternedTerms, 
    dbg_terms);

pub type Substs<I> = Terms<I>;

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
