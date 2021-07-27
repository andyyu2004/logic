use super::InferenceTable;
use logic_ir::{Interner, Subst, TyKind};

impl<I: Interner> InferenceTable<I> {
    pub(super) fn fresh_subst(&mut self, binders: &[()]) -> Subst<I> {
        let interner = self.interner;
        Subst::intern(
            interner,
            binders.iter().map(|_| TyKind::Infer(self.new_infer_var()).intern(interner)),
        )
    }
}
