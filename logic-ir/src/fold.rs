use crate::{Interner, LogicResult};
use logic_ir::*;

pub trait Fold<I: Interner>: Sized + Debug {
    type Folded: Debug = Self;
    fn fold_with<F: Folder<I>>(self, folder: &mut F) -> LogicResult<Self::Folded>;
}

pub trait FoldInner<I: Interner>: Fold<I> {
    fn fold_inner_with<F: Folder<I>>(self, folder: &mut F) -> LogicResult<Self::Folded>;
}

pub trait Folder<I: Interner>: Sized {
    fn interner(&self) -> I;

    fn fold_ty(&mut self, ty: Ty<I>) -> LogicResult<Ty<I>> {
        ty.fold_inner_with(self)
    }

    fn fold_infer_var(&mut self, infer: InferVar<I>) -> LogicResult<Ty<I>> {
        Ok(infer.to_ty(self.interner()))
    }

    fn fold<F: Fold<I>>(&mut self, foldable: F) -> LogicResult<F::Folded> {
        foldable.fold_with(self)
    }
}

impl<I: Interner> Fold<I> for Ty<I> {
    fn fold_with<F: Folder<I>>(self, folder: &mut F) -> LogicResult<Self::Folded> {
        folder.fold_ty(self)
    }
}

impl<I: Interner> FoldInner<I> for Ty<I> {
    fn fold_inner_with<F: Folder<I>>(self, folder: &mut F) -> LogicResult<Self::Folded> {
        let interner = folder.interner();
        let kind = match self.kind(interner) {
            // TODO bound properly
            TyKind::Infer(infer) => return folder.fold_infer_var(infer.clone()),
            TyKind::Bound(bound) => TyKind::Bound(bound.clone()),
            TyKind::Structure(f, xs) => TyKind::Structure(f.clone(), xs.clone().fold_with(folder)?),
        };
        Ok(kind.intern(interner))
    }
}

impl<I, T> Fold<I> for Canonical<T>
where
    I: Interner,
    T: Fold<I> + HasInterner<Interner = I>,
    T::Folded: HasInterner<Interner = I>,
{
    type Folded = Canonical<T::Folded>;

    fn fold_with<F: Folder<I>>(self, folder: &mut F) -> LogicResult<Self::Folded> {
        Ok(Canonical { value: self.value.fold_with(folder)?, binders: self.binders })
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

impl<I: Interner, T> Fold<I> for Binders<T>
where
    T: HasInterner<Interner = I> + Fold<I>,
    T::Folded: HasInterner<Interner = I>,
{
    type Folded = Binders<T::Folded>;

    fn fold_with<F: Folder<I>>(self, folder: &mut F) -> LogicResult<Self::Folded> {
        Ok(Binders::new(self.binders, self.value.fold_with(folder)?))
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
