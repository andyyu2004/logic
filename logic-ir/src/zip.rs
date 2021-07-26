use crate::*;

pub trait Zipper<I: Interner>: Sized {
    fn interner(&self) -> I;
    fn zip<Z: Zip<I>>(&mut self, a: &Z, b: &Z) -> LogicResult<()> {
        Zip::zip_with(self, a, b)
    }

    fn zip_tys(&mut self, a: &Ty<I>, b: &Ty<I>) -> LogicResult<()>;

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

impl<I: Interner> Zip<I> for GenericArgData<I> {
    fn zip_with<Z: Zipper<I>>(zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()> {
        match (a, b) {
            (GenericArgData::Ty(t), GenericArgData::Ty(u)) => Zip::zip_with(zipper, t, u),
        }
    }
}

impl<I: Interner> Zip<I> for ClauseData<I> {
    fn zip_with<Z: Zipper<I>>(zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()> {
        match (a, b) {
            (ClauseData::Implies(d0, g0), ClauseData::Implies(d1, g1)) => {
                Zip::zip_with(zipper, d0, d1)?;
                Zip::zip_with(zipper, g0, g1)
            }
        }
    }
}

impl<I: Interner> Zip<I> for Ty<I> {
    fn zip_with<Z: Zipper<I>>(zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()> {
        zipper.zip_tys(a, b)
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

impl<I: Interner> Zip<I> for Tys<I> {
    fn zip_with<Z: Zipper<I>>(zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()> {
        Zip::zip_with(zipper, a.as_slice(), b.as_slice())
    }
}

// delegate impl to it's data
macro_rules! zip_data {
    ($ty:ident) => {
        impl<I: Interner> Zip<I> for $ty<I> {
            fn zip_with<Z: Zipper<I>>(zipper: &mut Z, a: &Self, b: &Self) -> LogicResult<()> {
                Zip::zip_with(zipper, a.data(), b.data())
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

zip_data!(GenericArg);
zip_data!(Clause);
zip_data!(Goal);
