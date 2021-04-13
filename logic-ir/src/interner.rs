use crate::*;
use std::fmt::Debug;
use std::hash::Hash;

// the trait bounds are required as most types are parameterized by an interner
pub trait Interner: Copy + Eq + Hash + Debug {
    type InternedGoal: Clone + Eq + Hash + Debug;
    type InternedGoals: Clone + Eq + Hash + Debug;
    type InternedClause: Clone + Eq + Hash + Debug;
    type InternedClauses: Clone + Eq + Hash + Debug;

    type InternedTerm: GenericTerm<Self> + Clone + Eq + Hash + Debug;
    type InternedTerms: Clone + Eq + Hash + Debug;

    type DomainGoal: DomainGoal<Self, Self::InternedTerm> + Clone + Eq + Hash + Debug;

    fn goal_data<'a>(&self, goal: &'a Self::InternedGoal) -> &'a GoalData<Self>;
    fn goals<'a>(&self, goals: &'a Self::InternedGoals) -> &'a [Goal<Self>];
    fn intern_goal(self, goal: GoalData<Self>) -> Self::InternedGoal;
    fn intern_goals(self, goals: impl IntoIterator<Item = Goal<Self>>) -> Self::InternedGoals;

    fn clause_data<'a>(&self, clause: &'a Self::InternedClause) -> &'a ClauseData<Self>;
    fn clauses<'a>(&self, clauses: &'a Self::InternedClauses) -> &'a [Clause<Self>];
    fn intern_clause(self, clause: ClauseData<Self>) -> Self::InternedClause;
    fn intern_clauses(
        self,
        clauses: impl IntoIterator<Item = Clause<Self>>,
    ) -> Self::InternedClauses;

    fn term_data<'a, T: GenericTerm<Self>>(&self, terms: &'a Self::InternedTerm) -> &'a T;
    fn terms<'a, T: GenericTerm<Self>>(&self, terms: &'a Self::InternedTerms) -> &'a [Term<Self>];
    fn intern_term(self, term: impl GenericTerm<Self>) -> Self::InternedTerm;
    fn intern_terms<T: GenericTerm<Self>>(
        self,
        term: impl IntoIterator<Item = T>,
    ) -> Self::InternedTerms;
}
