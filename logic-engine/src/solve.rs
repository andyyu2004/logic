use crate::unify::InferCtxt;
use logic_ir::*;

pub struct RecursiveSolver<I: Interner> {
    pub interner: I,
    pub env: Environment<I>,
    pub infcx: InferCtxt<I>,
}

// TODO not sure what the solution type should be
type SolutionType<I> = Solution<I>;

impl<I: Interner> RecursiveSolver<I> {
    pub fn new(interner: I, env: Environment<I>) -> Self {
        Self { interner, env, infcx: InferCtxt::new(interner) }
    }

    pub fn solve(&self, goal: &Goal<I>) -> SolutionType<I> {
        match self.interner.goal_data(goal) {
            GoalData::DomainGoal(domain_goal) => self.solve_from_clauses(domain_goal),
            _ => self.simplify(goal),
        }
    }

    pub fn solve_from_clauses(&self, goal: &DomainGoal<I>) -> SolutionType<I> {
        for clause in &self.env.clauses {
            match clause.data() {
                ClauseData::Implies(consequent, condition) => {
                    // if !can_unify(clause, consequent) {
                    //     continue;
                    // }
                    todo!()
                }
            }
        }
        todo!()
    }

    pub fn simplify(&self, goal: &Goal<I>) -> SolutionType<I> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment<I: Interner> {
    clauses: Clauses<I>,
}

impl<I: Interner> Environment<I> {
    pub fn new(clauses: Clauses<I>) -> Self {
        Self { clauses }
    }
}

#[derive(Debug)]
pub enum Solution<I: Interner> {
    Unique(Subst<I>),
}
