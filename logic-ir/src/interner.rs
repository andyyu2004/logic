use crate::*;
use std::fmt::Debug;
use std::hash::Hash;

pub trait HasInterner {
    type Interner: Interner;
}

pub trait Internable = Clone + Eq + Hash + Debug;
// the trait bounds are required as most types are parameterized by an interner
// this has become a bit of a dumping ground for all types, probably not ideal
// e.g. UnificationContext doesn't really belong here but is here for technical reasons
pub trait Interner: Copy + Eq + Hash + Debug {
    type InternedGoal: Internable;
    type InternedGoals: Internable;
    type InternedClause: Internable;
    type InternedClauses: Internable;
    type InternedTy: Internable;
    type InternedSubst: Internable;
    type InternedVariables: Internable;

    fn goal_data<'a>(self, goal: &'a Self::InternedGoal) -> &'a GoalData<Self>;
    fn goals<'a>(self, goals: &'a Self::InternedGoals) -> &'a [Goal<Self>];
    fn intern_goal(self, goal: GoalData<Self>) -> Self::InternedGoal;
    fn intern_goals(self, goals: impl IntoIterator<Item = Goal<Self>>) -> Self::InternedGoals;

    fn clause_data<'a>(self, clause: &'a Self::InternedClause) -> &'a ClauseData<Self>;
    fn clauses<'a>(self, clauses: &'a Self::InternedClauses) -> &'a [Clause<Self>];
    fn intern_clause(self, clause: ClauseData<Self>) -> Self::InternedClause;
    fn intern_clauses(
        self,
        clauses: impl IntoIterator<Item = Clause<Self>>,
    ) -> Self::InternedClauses;

    fn ty_data<'a>(self, ty: &'a Self::InternedTy) -> &'a TyData<Self>;
    fn subst_data<'a>(self, tys: &'a Self::InternedSubst) -> &'a [Ty<Self>];
    fn intern_ty(self, ty: TyData<Self>) -> Self::InternedTy;
    fn intern_subst(self, tys: impl IntoIterator<Item = Ty<Self>>) -> Self::InternedSubst;

    fn variables<'a>(self, vars: &'a Self::InternedVariables) -> &'a [Variable<Self>];
    fn intern_variables<'a>(
        self,
        vars: impl IntoIterator<Item = Variable<Self>>,
    ) -> Self::InternedVariables;
}
