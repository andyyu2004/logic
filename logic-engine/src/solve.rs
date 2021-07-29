use crate::infer::{InferCtxt, InferenceTable};
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

    pub fn solve(&self, canonical_goal: &Canonical<Goal<I>>) -> SolutionResult<I> {
        debug!(canonical_goal = ?canonical_goal);
        let Canonical { value: goal, binders } = canonical_goal.clone();
        match goal.data(self.interner) {
            GoalData::DomainGoal(domain_goal) => {
                let canonical_domain_goal = Canonical { binders, value: domain_goal.clone() };
                self.solve_from_clauses(&canonical_domain_goal)
            }
            _ => self.simplify(canonical_goal),
        }
    }

    pub fn solve_from_clauses(
        &self,
        canonical_domain_goal: &Canonical<DomainGoal<I>>,
    ) -> SolutionResult<I> {
        let interner = self.interner;
        let mut current_solution: Option<Solution<I>> = None;
        for clause in &self.env.clauses {
            match clause.data(interner) {
                ClauseData::Implies(implication) => {
                    // println!("can unify {:?} and {:?}", goal, consequent);
                    // if !goal.can_unify(self.interner, consequent) {
                    //     println!("no");
                    //     continue;
                    // }

                    // temporary stuff to make stuff compile

                    let (infer, subst, goal) =
                        InferenceTable::from_canonical(interner, canonical_domain_goal.clone());

                    if let Ok(solution) =
                        InferCtxt::from_implication(self, infer, subst, goal, implication.clone())
                            .and_then(|infcx| infcx.solve())
                    {
                        match solution {
                            Solution::Unique(..) => match &current_solution {
                                // found two different solutions
                                Some(curr_sol) if &solution != curr_sol =>
                                    return Ok(Solution::Ambiguous),
                                _ => current_solution = Some(solution),
                            },
                            Solution::Ambiguous => continue,
                        }
                    }
                }
            }
        }

        match current_solution {
            Some(solution) => Ok(solution),
            None => Err(LogicError::NoSolution),
        }
    }

    pub fn simplify(&self, canonical_goal: &Canonical<Goal<I>>) -> SolutionResult<I> {
        let (infer, subst, goal) =
            InferenceTable::from_canonical(self.interner, canonical_goal.clone());
        InferCtxt::from_goal(self, infer, subst, goal)?.solve()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Solution<I: Interner> {
    Unique(Canonical<Subst<I>>),
    Ambiguous,
}

impl<I: Interner> Solution<I> {
    /// Returns `true` if the solution is [`Unique`].
    pub fn is_unique(&self) -> bool {
        matches!(self, Self::Unique(..))
    }

    pub fn into_unique(self) -> Canonical<Subst<I>> {
        if let Self::Unique(v) = self { v } else { panic!() }
    }
}

#[cfg(test)]
mod tests;
