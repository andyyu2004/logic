use crate::infer::{InferCtxt, InferenceTable};
use logic_ir::*;

impl<I: Interner> InferCtxt<'_, I> {
    pub fn unify<T: Zip<I>>(&mut self, a: &T, b: &T) -> LogicResult<()> {
        self.with_snapsnot(|infcx| Unifier { table: &mut infcx.table }.zip(a, b))
    }
}

pub struct Unifier<'i, I: Interner> {
    table: &'i mut InferenceTable<I>,
}

impl<I: Interner> Unifier<'_, I> {
    fn unify_ty_ty(&mut self, t: &Ty<I>, u: &Ty<I>) -> LogicResult<()> {
        let interner = self.interner();
        match (t.kind(interner), u.kind(interner)) {
            (TyKind::Structure(f, xs), TyKind::Structure(g, ys)) if f == g => self.zip(xs, ys),
            (&TyKind::Infer(i), &TyKind::Infer(j)) => Ok(self.unify_var_var(i, j)),
            (&TyKind::Infer(var), ..) => Ok(self.unify_var_ty(var, u.clone())?),
            (.., &TyKind::Infer(var)) => Ok(self.unify_var_ty(var, t.clone())?),
            _ => Err(LogicError::NoSolution),
        }
    }

    pub fn unify_var_ty(&mut self, var: InferVar<I>, ty: Ty<I>) -> LogicResult<()> {
        OccursCheck { interner: self.interner(), var }.fold(ty.clone())?;
        self.table
            .unify
            .unify_var_value(var, InferenceValue::Known(ty))
            .expect("should never fail");
        Ok(())
    }

    pub fn unify_var_var(&mut self, x: InferVar<I>, y: InferVar<I>) {
        self.table.unify.unify_var_var(x, y).unwrap()
    }
}

struct OccursCheck<I: Interner> {
    interner: I,
    var: InferVar<I>,
}

impl<I: Interner> Folder<I> for OccursCheck<I> {
    fn interner(&self) -> I {
        self.interner
    }

    fn fold_infer_var(&mut self, infer: InferVar<I>) -> LogicResult<Ty<I>> {
        if self.var == infer { Err(LogicError::NoSolution) } else { Ok(infer.to_ty(self.interner)) }
    }
}

impl<I: Interner> Zipper<I> for Unifier<'_, I> {
    fn interner(&self) -> I {
        self.table.interner
    }

    fn zip_tys(&mut self, t: &Ty<I>, u: &Ty<I>) -> LogicResult<()> {
        self.unify_ty_ty(t, u)
    }

    fn zip_binders<T>(&mut self, a: &Binders<T>, b: &Binders<T>) -> LogicResult<()>
    where
        T: HasInterner<Interner = I> + Zip<I>,
    {
        todo!()
    }
}
