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

    fn clause_data<'a>(&self, clause: &'a Self::InternedClause) -> &'a ClauseData<Self>;
    fn clauses<'a>(&self, clauses: &'a Self::InternedClauses) -> &'a [Clause<Self>];
    fn goal_data<'a>(&self, goal: &'a Self::InternedGoal) -> &'a GoalData<Self>;
    fn goals<'a>(&self, goals: &'a Self::InternedGoals) -> &'a [Goal<Self>];
    fn term_data<'a>(&self, term: &'a Self::InternedTerm) -> &'a TermData<Self>;
    fn terms<'a>(&self, terms: &'a Self::InternedTerms) -> &'a [Term<Self>];

    fn intern_goal(self, goal: GoalData<Self>) -> Self::InternedGoal;
    fn intern_clause(self, clause: ClauseData<Self>) -> Self::InternedClause;
    fn intern_term(self, term: TermData<Self>) -> Self::InternedTerm;

    fn intern_substs(self, subst: impl IntoIterator<Item = Term<Self>>) -> Self::InternedSubsts;

    fn intern_goals(self, goals: impl IntoIterator<Item = Goal<Self>>) -> Self::InternedGoals;

    fn intern_clauses(
        self,
        clauses: impl IntoIterator<Item = Clause<Self>>,
    ) -> Self::InternedClauses;

    fn intern_terms(self, terms: impl IntoIterator<Item = Term<Self>>) -> Self::InternedTerms;
}
