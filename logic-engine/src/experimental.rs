use crate::Environment;
use logic_ir::{Goal, GoalData, Interner};

pub struct Solver<I: Interner> {
    interner: I,
    env: Environment<I>,
}

pub struct Solution {}

impl<I: Interner> Solver<I> {
    pub fn solve_goal(&mut self, goal: Goal<I>) -> Solution {
        match goal.data() {
            GoalData::DomainGoal(_) => todo!(),
            GoalData::And(_, _) => todo!(),
            GoalData::Or(_, _) => todo!(),
            GoalData::Implies(_, _) => todo!(),
        }
    }
}
