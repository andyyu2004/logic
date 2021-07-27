use crate::*;

pub trait Zipper<I: Interner>: Sized {
    fn interner(&self) -> I;
    fn zip_tys(&mut self, a: &Ty<I>, b: &Ty<I>) -> LogicResult<()>;
    fn zip_binders<T>(&mut self, a: &Binders<T>, b: &Binders<T>) -> LogicResult<()>
    where
        T: HasInterner<Interner = I> + Zip<I>;

    fn zip<Z: Zip<I> + ?Sized>(&mut self, a: &Z, b: &Z) -> LogicResult<()> {
        Zip::zip_with(self, a, b)
    }

    fn zip_substs(&mut self, a: &Subst<I>, b: &Subst<I>) -> LogicResult<()>
    where
        Self: Sized,
    {
        for (a, b) in a.iter().zip(b.iter()) {
            Zip::zip_with(self, a, b)?;
        }
        Ok(())
    }
}

pub trait Zip<I>: Debug
where
    I: Interner,
{
    /// Uses the zipper to walk through two values, ensuring that they match.
    fn zip_with<Z: Zipper<I>>(zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()>;
}

impl<'a, T: ?Sized + Zip<I>, I: Interner> Zip<I> for &'a T {
    fn zip_with<Z: Zipper<I>>(zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()> {
        <T as Zip<I>>::zip_with(zipper, a, b)
    }
}

impl<I: Interner> Zip<I> for Ty<I> {
    fn zip_with<Z: Zipper<I>>(zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()> {
        zipper.zip_tys(a, b)
    }
}

impl<I: Interner, T> Zip<I> for Binders<T>
where
    T: HasInterner<Interner = I> + Zip<I>,
{
    fn zip_with<Z: Zipper<I>>(zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()> {
        zipper.zip_binders(a, b)
    }
}

impl<I: Interner, T: Zip<I>> Zip<I> for [T] {
    fn zip_with<Z: Zipper<I>>(zipper: &mut Z, xs: &Self, ys: &Self) -> LogicResult<()> {
        if xs.len() != ys.len() {
            return Err(LogicError::NoSolution);
        }

        for (x, y) in xs.iter().zip(ys) {
            zipper.zip(x, y)?;
        }

        Ok(())
    }
}

impl<I: Interner> Zip<I> for Subst<I> {
    fn zip_with<Z: Zipper<I>>(zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()> {
        Zip::zip_with(zipper, a.as_slice(), b.as_slice())
    }
}

// delegate impl to it's data
macro_rules! zip_data {
    ($ty:ident) => {
        impl<I: Interner> Zip<I> for $ty<I> {
            fn zip_with<Z: Zipper<I>>(zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()> {
                let interner = zipper.interner();
                Zip::zip_with(zipper, a.data(interner), b.data(interner))
            }
        }
    };
}

/// Generates a Zip impl that requires the two values be
/// equal. Suitable for atomic, scalar values.
macro_rules! zip_eq {
    ($ty:ty) => {
        impl<I: Interner> Zip<I> for $ty {
            fn zip_with<Z: Zipper<I>>(_zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()> {
                if a != b {
                    return Err(LogicError::NoSolution);
                }
                Ok(())
            }
        }
    };
}

zip_eq!(Ident);
zip_eq!(InferVar<I>);

zip_data!(Clause);
zip_data!(Goal);
