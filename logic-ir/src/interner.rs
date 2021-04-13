use crate::*;
use std::fmt::Debug;
use std::hash::Hash;

// the trait bounds are required as most types are parameterized by an interner
pub trait Interner: Copy + Eq + Hash + Debug {
    type InternedGoal: Clone + Eq + Hash + Debug;
    type InternedGoals: Clone + Eq + Hash + Debug;
    type InternedClause: Clone + Eq + Hash + Debug;
    type InternedClauses: Clone + Eq + Hash + Debug;

    // these should probably technically be generic parameters rather than associated types
    // but this is a bit disgusting as `where I: Interner` is likely to be everywhere
    // and we really shouldn't have to write something like
    // `where I: Interner<D: DomainGoal<I>, T: GenericTerm<I>>` or something
    // semantically, I think generics are the correct choice but its just too much
    // now we can only have one implementation of term per interner
    // but that's probaby not a big problem at all
    type ConcreteTerm: GenericTerm<Self> + Clone + Eq + Hash + Debug;
    type InternedTerm: Clone + Eq + Hash + Debug;
    type InternedTerms: Clone + Eq + Hash + Debug;

    type DomainGoal: DomainGoal<Self, Self::ConcreteTerm> + Clone + Eq + Hash + Debug;

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

    fn term_data<'a>(&self, terms: &'a Self::InternedTerm) -> &'a Self::ConcreteTerm;
    fn terms<'a>(&self, terms: &'a Self::InternedTerms) -> &'a [Term<Self>];
    fn intern_term(self, term: Self::ConcreteTerm) -> Self::InternedTerm;
    fn intern_terms(self, term: impl IntoIterator<Item = Term<Self>>) -> Self::InternedTerms;
}
