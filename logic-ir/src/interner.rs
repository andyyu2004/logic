use crate::*;
use std::fmt::Debug;
use std::hash::Hash;

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
    type InternedTys: Internable;
    type InternedGenericArg: Internable;
    type InternedSubst: Internable;

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
    fn tys<'a>(self, tys: &'a Self::InternedTys) -> &'a [Ty<Self>];
    fn intern_ty(self, ty: TyData<Self>) -> Self::InternedTy;
    fn intern_tys(self, tys: impl IntoIterator<Item = Ty<Self>>) -> Self::InternedTys;

    fn generic_arg_data<'a>(self, arg: &'a Self::InternedGenericArg) -> &'a GenericArgData<Self>;
    fn subst_data<'a>(self, subst: &'a Self::InternedSubst) -> &'a [GenericArg<Self>];
    fn intern_generic_arg(self, arg: GenericArgData<Self>) -> Self::InternedGenericArg;
    fn intern_subst(self, subst: impl IntoIterator<Item = GenericArg<Self>>)
    -> Self::InternedSubst;
}
