use logic_ir::*;

use crate::infer::InferenceTable;

pub trait GoalExt<I: Interner> {
    fn peel(self, interner: I) -> Canonical<Goal<I>>;
}

impl<I: Interner> GoalExt<I> for Goal<I> {
    fn peel(self, interner: I) -> Canonical<Goal<I>> {
        let mut infer = InferenceTable::new(interner);
        let mut goal = self;
        let peeled = loop {
            goal = match goal.data(interner) {
                GoalData::Quantified(Quantifier::Exists, quantified) =>
                    infer.instantiate(quantified.clone()),
                GoalData::Quantified(Quantifier::ForAll, _) => todo!(),
                _ => break goal,
            }
        };

        dbg!(&peeled);

        dbg!(infer.canonicalize(peeled))
    }
}
