use crate::*;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Interner: Copy + Eq + Ord + Hash + Debug {
    type InternedGoal: Clone + Eq + Hash + Debug;
    type InternedGoals: Clone + Eq + Hash + Debug;
    type InternedClause: Clone + Eq + Hash + Debug;
    type InternedClauses: Clone + Eq + Hash + Debug;
    type InternedTerm: Clone + Eq + Hash + Debug;
    type InternedTerms: Clone + Eq + Hash + Debug;
    type InternedSubsts: Clone + Eq + Hash + Debug;

    fn intern_goal(self, goal: Goal<Self>) -> Self::InternedGoal;
    fn intern_clause(self, clause: Clause<Self>) -> Self::InternedClause;
    fn intern_term(self, term: Term<Self>) -> Self::InternedTerm;

    fn intern_substs(
        self,
        subst: impl IntoIterator<Item = InternedTerm<Self>>,
    ) -> Self::InternedSubsts;

    fn intern_goals(
        self,
        goals: impl IntoIterator<Item = InternedGoal<Self>>,
    ) -> Self::InternedGoals;

    fn intern_clauses(
        self,
        clauses: impl IntoIterator<Item = InternedClause<Self>>,
    ) -> Self::InternedClauses;

    fn intern_terms(
        self,
        terms: impl IntoIterator<Item = InternedTerm<Self>>,
    ) -> Self::InternedTerms;
}
