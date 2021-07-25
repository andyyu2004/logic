#![feature(trait_alias)]

// https://www.youtube.com/watch?v=RwBiHLoQ3E4&ab_channel=PapersWeLove
mod ast_lowering;
mod debug;
mod interned;
mod interner;
mod unify;

pub mod tls;

pub use ast_lowering::{lower_ast, lower_goal};
pub use debug::DebugCtxt;
pub use interned::*;
pub use interner::Interner;
pub use logic_parse::{Ident, Symbol, Var};
use std::fmt::{self, Debug, Formatter};
pub use std::ops::{Deref, DerefMut};
use std::rc::Rc;

/// an interner that doesn't really intern anything
// the default "interner" for internal use
#[derive(Debug, Clone, Eq, PartialEq, Ord, Hash, PartialOrd, Copy)]
pub struct IRInterner;

impl Interner for IRInterner {
    // type DomainGoal = GenericTerm<Self>;
    // wrapped in `Rc` to make it cheaply cloneable
    // a proper interner should probably use copyable references
    type InternedClause = Rc<ClauseData<Self>>;
    type InternedClauses = Vec<Clause<Self>>;
    type InternedGenericArg = Rc<GenericArgData<Self>>;
    type InternedGoal = Rc<GoalData<Self>>;
    type InternedGoals = Vec<Goal<Self>>;
    type InternedSubst = Vec<GenericArg<Self>>;
    type InternedTy = Rc<TyData<Self>>;
    type InternedTys = Vec<Ty<Self>>;

    fn goal_data<'a>(self, goal: &'a Self::InternedGoal) -> &'a GoalData<Self> {
        goal
    }

    fn goals<'a>(self, goals: &'a Self::InternedGoals) -> &'a [Goal<Self>] {
        goals.as_slice()
    }

    fn intern_goal(self, goal: GoalData<Self>) -> Self::InternedGoal {
        Rc::new(goal)
    }

    fn intern_goals(self, goals: impl IntoIterator<Item = Goal<Self>>) -> Self::InternedGoals {
        goals.into_iter().collect()
    }

    fn clause_data<'a>(self, clause: &'a Self::InternedClause) -> &'a ClauseData<Self> {
        clause
    }

    fn clauses<'a>(self, clauses: &'a Self::InternedClauses) -> &'a [Clause<Self>] {
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

    fn ty_data<'a>(self, tys: &'a Self::InternedTy) -> &'a TyData<Self> {
        tys
    }

    fn tys<'a>(self, tys: &'a Self::InternedTys) -> &'a [Ty<Self>] {
        tys.as_slice()
    }

    fn intern_ty(self, ty: TyData<Self>) -> Self::InternedTy {
        Rc::new(ty)
    }

    fn intern_tys(self, term: impl IntoIterator<Item = Ty<Self>>) -> Self::InternedTys {
        term.into_iter().collect()
    }

    fn intern_generic_arg(self, arg: GenericArgData<Self>) -> Self::InternedGenericArg {
        Rc::new(arg)
    }

    fn intern_subst(
        self,
        subst: impl IntoIterator<Item = GenericArg<Self>>,
    ) -> Self::InternedSubst {
        subst.into_iter().collect()
    }

    fn generic_arg_data<'a>(self, arg: &'a Self::InternedGenericArg) -> &'a GenericArgData<Self> {
        arg
    }

    fn subst_data<'a>(self, subst: &'a Self::InternedSubst) -> &'a [GenericArg<Self>] {
        subst.as_slice()
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
    DomainGoal(DomainGoal<I>),
    And(Goal<I>, Goal<I>),
    Or(Goal<I>, Goal<I>),
    Implies(Clause<I>, Goal<I>),
}

impl<I: Interner> Debug for GoalData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DomainGoal(domain_goal) => write!(f, "{:?}", domain_goal),
            _ => todo!(),
        }
    }
}

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct TyData<I: Interner> {
    kind: TyKind<I>,
}

impl<I: Interner> Debug for TyData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

#[derive(Hash, Clone, PartialEq, Eq)]
pub enum TyKind<I: Interner> {
    Something(Subst<I>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericArgData<I: Interner> {
    Ty(Ty<I>),
}

impl<I: Interner> Debug for TyKind<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum DomainGoal<I: Interner> {
    Holds(Constraint<I>),
}

impl<I: Interner> Debug for DomainGoal<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DomainGoal::Holds(constraint) => write!(f, "{:?}", constraint),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Constraint<I: Interner> {
    Implemented(ImplConstraint<I>),
}

impl<I: Interner> Debug for Constraint<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Constraint::Implemented(impl_constraint) => write!(f, "{:?}", impl_constraint),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ImplConstraint<I: Interner> {
    ty: Ty<I>,
    trait_ref: TraitRef<I>,
}

impl<I: Interner> Debug for ImplConstraint<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.ty, self.trait_ref)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TraitRef<I> {
    pd: std::marker::PhantomData<I>,
}

impl<I: Interner> Debug for TraitRef<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ClauseData<I: Interner> {
    /// <clause> :- <goals>
    /// empty goals means the implication is a fact
    Implies(DomainGoal<I>, Goals<I>),
    // todo forall
}

impl<I: Interner> Debug for ClauseData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ClauseData::Implies(consequent, conditions) =>
                if conditions.is_empty() {
                    write!(f, "{:?}", consequent)
                } else {
                    write!(f, "{:?} :- {:?}", consequent, conditions)
                },
        }
    }
}
