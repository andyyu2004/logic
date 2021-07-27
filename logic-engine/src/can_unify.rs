use logic_ir::*;

pub trait CanUnify<I: Interner> {
    fn can_unify(&self, interner: I, other: &Self) -> bool;
}

struct MatchZipper<I> {
    interner: I,
}

impl<I: Interner> Zipper<I> for MatchZipper<I> {
    fn interner(&self) -> I {
        self.interner
    }

    fn zip_tys(&mut self, a: &Ty<I>, b: &Ty<I>) -> LogicResult<()> {
        let interner = self.interner();
        let can_match = match (a.kind(interner), b.kind(interner)) {
            (TyKind::Structure(f, xs), TyKind::Structure(g, ys)) =>
                f.can_unify(interner, g) && xs.can_unify(interner, ys),
            (TyKind::Structure(_, _), TyKind::Infer(_)) => todo!(),
            (TyKind::Infer(_), TyKind::Structure(_, _)) => todo!(),
            (TyKind::Infer(_), TyKind::Infer(_)) => todo!(),
        };

        if can_match { Ok(()) } else { Err(LogicError::NoSolution) }
    }

    fn zip_binders<T>(
        &mut self,
        a: &logic_ir::Binders<T>,
        b: &logic_ir::Binders<T>,
    ) -> LogicResult<()>
    where
        T: HasInterner<Interner = I> + Zip<I>,
    {
        todo!()
    }
}

impl<I, T> CanUnify<I> for T
where
    I: Interner,
    T: Zip<I> + ?Sized,
{
    fn can_unify(&self, interner: I, other: &Self) -> bool {
        MatchZipper { interner }.zip(self, other).is_ok()
    }
}
