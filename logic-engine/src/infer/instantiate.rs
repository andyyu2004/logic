use super::InferenceTable;
use logic_ir::*;

impl<I: Interner> InferenceTable<I> {
    pub(super) fn fresh_subst(&mut self, binders: &[Variable<I>]) -> Subst<I> {
        let interner = self.interner;
        Subst::intern(
            interner,
            binders.iter().map(|_| TyKind::Infer(self.new_infer_var()).intern(interner)),
        )
    }

    pub fn instantiate<T>(&mut self, bound: Binders<T>) -> T::Folded
    where
        T: Fold<I> + HasInterner<Interner = I>,
    {
        let (binders, value) = bound.split();
        let subst = self.fresh_subst(binders.as_slice());
        subst.apply(self.interner, value)
    }
}
