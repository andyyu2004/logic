use crate::{Interner, LogicResult};
use logic_ir::*;

pub trait Fold<I: Interner>: Sized {
    type Folded = Self;
    fn fold_with<F: Folder<I>>(self, folder: &mut F) -> LogicResult<Self::Folded>;
}

pub trait Folder<I: Interner> {
    fn interner(&self) -> I;
    fn fold_ty(&mut self, ty: Ty<I>) -> LogicResult<Ty<I>>;
}

impl<I: Interner> Fold<I> for Ty<I> {
    fn fold_with<F: Folder<I>>(self, folder: &mut F) -> LogicResult<Self::Folded> {
        folder.fold_ty(self)
    }
}

impl<I: Interner> Fold<I> for Subst<I> {
    fn fold_with<F: Folder<I>>(self, folder: &mut F) -> LogicResult<Self::Folded> {
        let interner = folder.interner();
        let new_subst =
            self.iter().cloned().map(|arg| arg.fold_with(folder)).collect::<Result<Vec<_>, _>>()?;
        Ok(Subst::intern(interner, new_subst))
    }
}

macro_rules! fold_interned {
    ($interned:ty) => {
        impl<I: Interner> Fold<I> for $interned {
            fn fold_with<F: Folder<I>>(self, folder: &mut F) -> LogicResult<Self::Folded> {
                let interner = folder.interner();
                let folded = self.data(interner).clone().fold_with(folder)?;
                Ok(Self::intern(interner, folded))
            }
        }
    };
}

fold_interned!(Clause<I>);
fold_interned!(Goal<I>);

macro_rules! copy_fold {
    ($t:ty) => {
        impl<I: Interner> $crate::fold::Fold<I> for $t {
            type Folded = Self;

            fn fold_with<F: $crate::fold::Folder<I>>(
                self,
                _folder: &mut F,
            ) -> $crate::LogicResult<Self::Folded> {
                Ok(self)
            }
        }
    };
}

copy_fold!(Ident);
