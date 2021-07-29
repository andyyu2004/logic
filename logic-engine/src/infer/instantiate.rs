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

    fn instantiate_binders<T>(&mut self, binders: Variables<I>, value: T) -> T::Folded
    where
        T: Fold<I>,
    {
        let subst = self.fresh_subst(binders.as_slice());
        subst.apply(self.interner, value)
    }

    pub fn instantiate_canonical<T: HasInterner<Interner = I>>(
        &mut self,
        canonical: Canonical<T>,
    ) -> T::Folded
    where
        T: Fold<I> + HasInterner<Interner = I>,
    {
        let Canonical { binders, value } = canonical;
        self.instantiate_binders(binders, value)
    }

    /// instantiate bound value existentially
    pub fn instantiate<T>(&mut self, bound: Binders<T>) -> T::Folded
    where
        T: Fold<I> + HasInterner<Interner = I>,
    {
        let (binders, value) = bound.split();
        self.instantiate_binders(binders, value)
    }
}
