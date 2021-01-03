pub mod db;
mod unify;

use ir::*;

pub trait Solver<I: Interner> {
    fn solve(&mut self, db: &db::Database, env: Environment<I>, goal: &GoalData<I>) -> Solution<I> {
        todo!()
    }
}

pub struct RecursiveSolver<I: Interner> {
    pub interner: I,
    pub env: Environment<I>,
}

impl<I: Interner> RecursiveSolver<I> {
    fn solve(&self, goal: &GoalData<I>) -> Option<Solution<I>> {
        match goal {
            GoalData::Term(term) => {
                for clause in self.interner.clauses(&self.env.clauses) {
                    let clause = self.interner.clause_data(clause);
                    match clause {
                        ClauseData::Horn(consequent, conditions) =>
                            if consequent == term {
                                let conditions = self.interner.goals(conditions);
                                for condition in conditions {
                                    if self.solve(self.interner.goal_data(condition)).is_none() {
                                        continue;
                                    }
                                }
                                return Some(Solution::Todo);
                            },
                    };
                }
                return None;
            }
            GoalData::And(..) | GoalData::Or(..) => todo!(),
        }
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

pub enum Solution<I: Interner> {
    Unique(Substs<I>),
    Todo,
}
