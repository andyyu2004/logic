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
        infcx.unify(&domain_goal, &instantiated.consequent)?;
        debug!("unified successful");
        infcx.obligations.push(Obligation::Prove(instantiated.condition));
        Ok(infcx)
    }

    fn prove(&mut self, subgoal: Goal<I>) -> LogicResult<Solution<I>> {
        debug!(subgoal = ?subgoal);
        let canonical_subgoal = self.canonicalize(subgoal);
        let solution = self.solver.solve(&canonical_subgoal)?;
        debug!(subgoal_solution = ?solution);
        Ok(solution)
    }

    pub fn solve(mut self) -> LogicResult<Solution<I>> {
        while let Some(obligation) = self.obligations.pop() {
            debug!(obligation = ?obligation);
            match obligation {
                Obligation::Prove(goal) => match self.prove(goal)? {
                    Solution::Unique(solution_subst) =>
                        solution_subst.apply(self.interner, self.subst.clone()),
                    Solution::Ambiguous => todo!(),
                },
            };
        }
        debug!(solution_subst = ?self.subst);
        Ok(Solution::Unique(self.subst))
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
