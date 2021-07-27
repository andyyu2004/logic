use super::InferenceTable;
use logic_ir::*;

#[derive(Debug, Clone)]
pub struct Canonical<T> {
    pub value: T,
}

impl<I: Interner, T: Fold<I>> Fold<I> for Canonical<T> {
    type Folded = Canonical<T::Folded>;

    fn fold_with<F: Folder<I>>(self, folder: &mut F) -> LogicResult<Self::Folded> {
        Ok(Canonical { value: self.value.fold_with(folder)? })
    }
}

impl<I: Interner> InferenceTable<I> {
    pub fn canonicalize<T>(&mut self, value: T) -> Canonical<T::Folded>
    where
        T: Fold<I>,
    {
        let value = value.fold_with(&mut Canonicalizer { interner: self.interner }).unwrap();
        Canonical { value }
    }
}

pub struct Canonicalizer<I: Interner> {
    interner: I,
}

impl<I: Interner> Folder<I> for Canonicalizer<I> {
    fn interner(&self) -> I {
        self.interner
    }

    fn fold_ty(&mut self, ty: Ty<I>) -> LogicResult<Ty<I>> {
        // TODO
        Ok(ty)
    }
}
