#![feature(trait_alias)]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate logic_derive;

// for proc macro to be able to refer to this crate
extern crate self as logic_ir;

// https://www.youtube.com/watch?v=RwBiHLoQ3E4&ab_channel=PapersWeLove
mod ast_lowering;
mod debug;
mod interned;
mod interner;

mod fold;
mod subst;
pub mod tls;
pub mod zip;

pub use fold::*;
pub use subst::*;
pub use zip::*;

pub use ast_lowering::{lower_ast, lower_goal};
pub use debug::DebugCtxt;
use indexed_vec::{newtype_index, Idx};
pub use interned::*;
pub use interner::Interner;
pub use logic_parse::{Ident, Symbol, Var};
use std::fmt::{self, Debug, Display, Formatter};
pub use std::ops::{Deref, DerefMut};
use std::rc::Rc;

/// an interner that doesn't really intern anything
// the default "interner" for internal use
#[derive(Debug, Clone, Eq, PartialEq, Ord, Hash, PartialOrd, Copy)]
pub struct LogicInterner;

impl Interner for LogicInterner {
    // type DomainGoal = GenericTerm<Self>;
    // wrapped in `Rc` to make it cheaply cloneable
    // a proper interner should probably use copyable references
    type InternedClause = Rc<ClauseData<Self>>;
    type InternedClauses = Vec<Clause<Self>>;
    type InternedGoal = Rc<GoalData<Self>>;
    type InternedGoals = Vec<Goal<Self>>;
    type InternedSubst = Vec<Ty<Self>>;
    type InternedTy = Rc<TyData<Self>>;

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

    fn subst_data<'a>(self, subst: &'a Self::InternedSubst) -> &'a [Ty<Self>] {
        subst.as_slice()
    }

    fn intern_ty(self, ty: TyData<Self>) -> Self::InternedTy {
        Rc::new(ty)
    }

    fn intern_subst(self, subst: impl IntoIterator<Item = Ty<Self>>) -> Self::InternedSubst {
        subst.into_iter().collect()
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
#[derive(Clone, PartialEq, Eq, Hash, HasInterner, Zip, Fold)]
pub enum GoalData<I: Interner> {
    DomainGoal(DomainGoal<I>),
    And(Goal<I>, Goal<I>),
    Or(Goal<I>, Goal<I>),
    Implies(Clause<I>, Goal<I>),
    True,
}

impl<I: Interner> Debug for GoalData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DomainGoal(domain_goal) => write!(f, "{:?}", domain_goal),
            _ => todo!(),
        }
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Zip)]
pub struct TyData<I: Interner> {
    // todo tyflags
    kind: TyKind<I>,
}

impl<I: Interner> TyData<I> {
    pub fn new(kind: TyKind<I>) -> Self {
        Self { kind }
    }
}

impl<I: Interner> Debug for TyData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Zip)]
pub enum TyKind<I: Interner> {
    Structure(Ident, Subst<I>),
    Infer(InferVar<I>),
}

newtype_index!(InferIdx);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InferVar<I: Interner> {
    idx: InferIdx,
    phantom: std::marker::PhantomData<I>,
}

impl<I: Interner> ena::unify::UnifyKey for InferVar<I> {
    type Value = InferenceValue<I>;

    fn index(&self) -> u32 {
        self.idx.index() as u32
    }

    fn from_index(idx: u32) -> Self {
        Self::new(InferIdx::new(idx as usize))
    }

    fn tag() -> &'static str {
        "InferenceVar"
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InferenceValue<I: Interner> {
    Known(Ty<I>),
    Unknown,
}

impl<I: Interner> ena::unify::UnifyValue for InferenceValue<I> {
    type Error = ena::unify::NoError;

    /// Given two values, produce a new value that combines them.
    /// If that is not possible, produce an error.
    fn unify_values(x: &Self, y: &Self) -> Result<Self, Self::Error> {
        Ok(match (x, y) {
            (Self::Known(..), Self::Known(..)) => panic!("unifying two known values"),
            (Self::Known(..), Self::Unknown) => x.clone(),
            (Self::Unknown, Self::Known(..)) => y.clone(),
            (Self::Unknown, Self::Unknown) => Self::Unknown,
        })
    }
}

impl<I: Interner> InferVar<I> {
    pub fn new(idx: InferIdx) -> Self {
        Self { idx, phantom: std::marker::PhantomData }
    }
}

impl<I: Interner> Debug for InferVar<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "?{:?}", self.idx)
    }
}

impl<I: Interner> TyKind<I> {
    pub fn intern(self, interner: I) -> Ty<I> {
        Ty::intern(interner, TyData::new(self))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Fold, Zip)]
pub enum GenericArgData<I: Interner> {
    Ty(Ty<I>),
}

impl<I: Interner> Debug for TyKind<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TyKind::Structure(functor, args) =>
                write!(f, "{}{}", functor, util::fmt_generic_args(args.as_slice())),
            TyKind::Infer(var) => write!(f, "{:?}", var),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, HasInterner, Zip, Fold)]
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

#[derive(Clone, PartialEq, Eq, Hash, Zip, Fold, HasInterner)]
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

#[derive(Clone, PartialEq, Eq, Hash, Zip, Fold, HasInterner)]
pub struct ImplConstraint<I: Interner> {
    ty: Ty<I>,
    trait_ref: TraitRef<I>,
}

impl<I: Interner> Debug for ImplConstraint<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.ty, self.trait_ref)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, HasInterner, Zip, Fold)]
pub struct TraitRef<I: Interner> {
    pub trait_name: Ident,
    pub args: Subst<I>,
}

impl<I: Interner> Debug for TraitRef<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.trait_name, util::fmt_generic_args(self.args.as_slice()))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Zip, Fold)]
pub enum ClauseData<I: Interner> {
    /// <clause> :- <goal>
    Implies(Implication<I>),
}

impl<I: Interner> Debug for ClauseData<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ClauseData::Implies(implication) => write!(f, "{:?}", implication),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Zip, Fold)]
pub struct Implication<I: Interner> {
    pub consequent: DomainGoal<I>,
    pub condition: Goal<I>,
}

impl<I: Interner> Debug for Implication<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} :- {:?}", self.consequent, self.condition)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment<I: Interner> {
    pub clauses: Clauses<I>,
}

impl<I: Interner> Environment<I> {
    pub fn new(clauses: Clauses<I>) -> Self {
        Self { clauses }
    }
}

pub type LogicResult<T> = Result<T, LogicError>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LogicError {
    NoSolution,
}

impl Display for LogicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
