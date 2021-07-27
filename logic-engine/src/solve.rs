use crate::infer::{Canonical, InferCtxt, InferenceTable};
use logic_ir::*;

#[derive(Debug)]
pub struct RecursiveSolver<I: Interner> {
    pub interner: I,
    pub env: Environment<I>,
}

type SolutionResult<I> = LogicResult<Solution<I>>;

impl<I: Interner> RecursiveSolver<I> {
    pub fn new(interner: I, env: Environment<I>) -> Self {
        Self { interner, env }
    }

    pub fn solve(&self, goal: &Goal<I>) -> SolutionResult<I> {
        match self.interner.goal_data(goal) {
            GoalData::DomainGoal(domain_goal) => self.solve_from_clauses(domain_goal),
            _ => self.simplify(goal),
        }
    }

    pub fn solve_from_clauses(&self, domain_goal: &DomainGoal<I>) -> SolutionResult<I> {
        let interner = self.interner;
        let mut current_solution = None;
        for clause in &self.env.clauses {
            match clause.data(interner) {
                ClauseData::Implies(implication) => {
                    // println!("can unify {:?} and {:?}", goal, consequent);
                    // if !goal.can_unify(self.interner, consequent) {
                    //     println!("no");
                    //     continue;
                    // }

                    // temporary stuff to make stuff compile
                    let canonical = Canonical { value: domain_goal.clone() };

                    let (infer, subst, goal) = InferenceTable::from_canonical(interner, canonical);

                    let infcx = InferCtxt::from_implication(
                        self,
                        infer,
                        subst,
                        Canonical { value: goal },
                        implication.clone(),
                    )?;

                    let solution = infcx.solve()?;
                    match solution {
                        Solution::Unique(..) => match current_solution.take() {
                            Some(..) => return Ok(Solution::Ambiguous),
                            None => current_solution = Some(solution),
                        },
                        Solution::Ambiguous => continue,
                    }
                }
            }
        }

        match current_solution {
            Some(solution) => Ok(solution),
            None => Err(LogicError::NoSolution),
        }
    }

    pub fn simplify(&self, goal: &Goal<I>) -> SolutionResult<I> {
        match goal.data(self.interner) {
            GoalData::DomainGoal(_) => todo!(),
            GoalData::And(_, _) => todo!(),
            GoalData::Or(_, _) => todo!(),
            GoalData::Implies(_, _) => todo!(),
            GoalData::True => Ok(Solution::Unique(Subst::empty(self.interner))),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Solution<I: Interner> {
    Unique(Subst<I>),
    Ambiguous,
}

#[cfg(test)]
mod tests;
