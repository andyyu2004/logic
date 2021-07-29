mod canonical;
mod instantiate;

pub use canonical::*;
// use indexed_vec::Idx;
use crate::{RecursiveSolver, Solution};
use logic_ir::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Obligation<I: Interner> {
    Prove(Goal<I>),
}

#[derive(Debug)]
pub struct InferCtxt<'a, I: Interner> {
    pub(super) table: InferenceTable<I>,
    obligations: Vec<Obligation<I>>,
    solver: &'a RecursiveSolver<I>,
    subst: Subst<I>,
}

impl<'a, I: Interner> Deref for InferCtxt<'a, I> {
    type Target = InferenceTable<I>;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl<'a, I: Interner> DerefMut for InferCtxt<'a, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

#[derive(Debug)]
pub struct InferenceTable<I: Interner> {
    pub interner: I,
    pub(super) unify: ena::unify::InPlaceUnificationTable<InferVar<I>>,
    vars: Vec<InferVar<I>>,
}

impl<I: Interner> InferenceTable<I> {
    pub fn new(interner: I) -> Self {
        Self { interner, unify: Default::default(), vars: Default::default() }
    }

    pub(super) fn new_infer_var(&mut self) -> InferVar<I> {
        self.unify.new_key(InferenceValue::Unknown)
    }

    pub fn probe_var(&mut self, infer: InferVar<I>) -> Option<Ty<I>> {
        match self.unify.probe_value(infer) {
            InferenceValue::Known(ty) => Some(ty),
            InferenceValue::Unknown => None,
        }
    }

    pub fn from_canonical<T>(interner: I, canonical: Canonical<T>) -> (Self, Subst<I>, T)
    where
        T: Fold<I, Folded = T>,
        T::Folded: HasInterner<Interner = I>,
    {
        let mut table = Self::new(interner);

        let fresh_subst = table.fresh_subst(canonical.binders.as_slice());
        let value = fresh_subst.apply(interner, canonical.value);

        (table, fresh_subst, value)
    }
}

pub struct InferCtxtSnapshot<I: Interner> {
    table_snapshot: ena::unify::Snapshot<ena::unify::InPlace<InferVar<I>>>,
}

struct PositiveSolution<I: Interner> {
    original_vars: Vec<InferVar<I>>,
    solution: Solution<I>,
}

impl<'a, I: Interner> InferCtxt<'a, I> {
    pub fn from_implication(
        solver: &'a RecursiveSolver<I>,
        table: InferenceTable<I>,
        subst: Subst<I>,
        domain_goal: DomainGoal<I>,
        implication: Binders<Implication<I>>,
    ) -> LogicResult<Self> {
        let mut infcx = Self { solver, subst, table, obligations: vec![] };
        debug!(implication = ?implication);
        let instantiated = infcx.instantiate(implication);
        debug!(instantiated = ?instantiated);
        debug!("try unify `{:?}` with `{:?}`", instantiated.consequent, domain_goal);
        match infcx.unify(&domain_goal, &instantiated.consequent) {
            Ok(()) => {
                debug!("unified successfully");
            }
            Err(err) => {
                debug!("not unifiable");
                return Err(err);
            }
        }
        infcx.add_goal(instantiated.condition)?;
        Ok(infcx)
    }

    pub fn from_goal(
        solver: &'a RecursiveSolver<I>,
        table: InferenceTable<I>,
        subst: Subst<I>,
        goal: Goal<I>,
    ) -> LogicResult<Self> {
        let mut infcx = Self { solver, subst, table, obligations: vec![] };
        infcx.add_goal(goal)?;
        Ok(infcx)
    }

    pub fn solve(mut self) -> LogicResult<Solution<I>> {
        while let Some(obligation) = self.obligations.pop() {
            debug!(remaining_obligation = ?self.obligations);
            debug!(obligation = ?obligation);
            match obligation {
                Obligation::Prove(goal) => {
                    let PositiveSolution { original_vars, solution } = self.prove(goal)?;
                    match solution {
                        Solution::Unique(subst) => self.apply_solution(original_vars, subst)?,
                        Solution::Ambiguous => todo!(),
                    }
                }
            };
        }

        let subst = self.subst.clone();
        let canonical_subst = self.canonicalize(subst);
        debug!(solution_subst = ?canonical_subst);
        Ok(Solution::Unique(canonical_subst))
    }

    fn prove(&mut self, subgoal: Goal<I>) -> LogicResult<PositiveSolution<I>> {
        debug!(subgoal = ?subgoal);
        let Canonicalized { canonical, original_vars } = self.canonicalized(subgoal);
        let solution = self.solver.solve(&canonical)?;
        debug!(subgoal_solution = ?solution);
        Ok(PositiveSolution { solution, original_vars })
    }

    // add and simplify goal
    fn add_goal(&mut self, goal: Goal<I>) -> LogicResult<()> {
        match goal.data(self.interner) {
            GoalData::DomainGoal(..) => self.obligations.push(Obligation::Prove(goal)),
            GoalData::Quantified(Quantifier::Exists, quantified_goal) => {
                let instantiated_goal = self.instantiate(quantified_goal.clone());
                self.add_goal(instantiated_goal)?;
            }
            GoalData::Quantified(Quantifier::ForAll, _) => todo!(),
            GoalData::And(lhs, rhs) => {
                self.add_goal(lhs.clone())?;
                self.add_goal(rhs.clone())?;
            }
            GoalData::Implies(_, _) => todo!(),
            GoalData::Or(_, _) => todo!(),
            GoalData::True => {}
        };
        Ok(())
    }

    fn apply_solution(
        &mut self,
        original_vars: Vec<InferVar<I>>,
        canonical_subst: Canonical<Subst<I>>,
    ) -> LogicResult<()> {
        let subst = self.instantiate_canonical(canonical_subst);
        for (var, ty) in original_vars.into_iter().zip(subst.as_slice()) {
            self.unify.unify_var_value(var, InferenceValue::Known(ty.clone())).unwrap();
        }

        Ok(())
    }

    pub fn snapshot(&mut self) -> InferCtxtSnapshot<I> {
        let snapshot = self.table.unify.snapshot();
        InferCtxtSnapshot { table_snapshot: snapshot }
    }

    pub fn rollback_to(&mut self, snapshot: InferCtxtSnapshot<I>) {
        self.table.unify.rollback_to(snapshot.table_snapshot)
    }

    pub fn with_snapsnot<R>(
        &mut self,
        f: impl FnOnce(&mut Self) -> LogicResult<R>,
    ) -> LogicResult<R> {
        let snapshot = self.snapshot();
        match f(self) {
            Ok(r) => {
                self.table.unify.commit(snapshot.table_snapshot);
                Ok(r)
            }
            Err(err) => {
                self.rollback_to(snapshot);
                Err(err)
            }
        }
    }
}
