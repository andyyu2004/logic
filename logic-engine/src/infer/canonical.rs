use super::*;

impl<I: Interner> InferenceTable<I> {
    pub fn canonicalize<T>(&mut self, value: T) -> Canonical<T::Folded>
    where
        T: Fold<I>,
        T::Folded: HasInterner<Interner = I>,
    {
        let mut canonicalizer = Canonicalizer::new(self);
        let canonical = value.fold_with(&mut canonicalizer).unwrap();
        let binders = canonicalizer.binders();
        Canonical { value: canonical, binders }
    }
}

// replace unbound inference variables with "canonical variables"
pub struct Canonicalizer<'a, I: Interner> {
    table: &'a mut InferenceTable<I>,
    canonical_vars: Vec<InferVar<I>>,
}

impl<'a, I: Interner> Canonicalizer<'a, I> {
    pub fn new(table: &'a mut InferenceTable<I>) -> Self {
        Self { table, canonical_vars: Default::default() }
    }

    fn binders(&self) -> Variables<I> {
        Variables::intern(self.interner(), self.canonical_vars.iter().map(|_| Variable::new()))
    }

    fn add_canonical_var(&mut self, var: InferVar<I>) -> usize {
        self.canonical_vars.iter().position(|v| v == &var).unwrap_or_else(|| {
            let next = self.canonical_vars.len();
            self.canonical_vars.push(var);
            next
        })
    }
}

impl<'a, I: Interner> Folder<I> for Canonicalizer<'a, I> {
    #[inline]
    fn interner(&self) -> I {
        self.table.interner
    }

    fn fold_infer_var(&mut self, infer: InferVar<I>) -> LogicResult<Ty<I>> {
        match self.table.probe_var(infer) {
            Some(ty) => ty.fold_with(self),
            None => {
                let root = self.table.unify.find(infer);
                let index = self.add_canonical_var(root);
                Ok(BoundVar::new(DebruijnIdx::ZERO, index).to_ty(self.interner()))
            }
        }
    }
}
