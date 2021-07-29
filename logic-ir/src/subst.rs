use crate::*;

pub trait Substitute<I: Interner>: Fold<I> {
    fn subst(self, interner: I, subst: &Subst<I>) -> Self::Folded;
}

struct SubstFolder<'a, I: Interner> {
    interner: I,
    subst: &'a Subst<I>,
}

impl<I: Interner> Folder<I> for SubstFolder<'_, I> {
    fn interner(&self) -> I {
        self.interner
    }

    fn fold_ty(&mut self, ty: Ty<I>) -> LogicResult<Ty<I>> {
        match ty.kind(self.interner) {
            // TODO not a correct impl
            TyKind::Bound(BoundVar { debruijn, index }) =>
                Ok(self.subst.as_slice()[*index].clone()),
            _ => ty.fold_inner_with(self),
        }
    }
}

impl<I: Interner> Subst<I> {
    pub fn apply<T: Fold<I>>(&self, interner: I, value: T) -> T::Folded {
        value.subst(interner, self)
    }
}

impl<I: Interner, T: Fold<I>> Substitute<I> for T {
    fn subst(self, interner: I, subst: &Subst<I>) -> Self::Folded {
        self.fold_with(&mut SubstFolder { interner, subst }).unwrap()
    }
}
