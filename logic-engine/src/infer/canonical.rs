use super::*;

pub struct Canonicalized<T: HasInterner> {
    pub canonical: Canonical<T>,
    pub original_vars: Vec<InferVar<T::Interner>>,
}

impl<I: Interner> InferenceTable<I> {
    pub fn canonicalized<T>(&mut self, value: T) -> Canonicalized<T::Folded>
    where
        T: Fold<I>,
        T::Folded: HasInterner<Interner = I>,
    {
        let mut canonicalizer = Canonicalizer::new(self);
        let canonicalized = value.fold_with(&mut canonicalizer).unwrap();
        let binders = canonicalizer.binders();
        let canonical = Canonical { value: canonicalized, binders };
        Canonicalized { canonical, original_vars: canonicalizer.original_vars }
    }

    pub fn canonicalize<T>(&mut self, value: T) -> Canonical<T::Folded>
    where
        T: Fold<I>,
        T::Folded: HasInterner<Interner = I>,
    {
        self.canonicalized(value).canonical
    }
}

// replace unbound inference variables with "canonical variables"
pub struct Canonicalizer<'a, I: Interner> {
    table: &'a mut InferenceTable<I>,
    original_vars: Vec<InferVar<I>>,
}

impl<'a, I: Interner> Canonicalizer<'a, I> {
    pub fn new(table: &'a mut InferenceTable<I>) -> Self {
        Self { table, original_vars: Default::default() }
    }

    fn binders(&self) -> Variables<I> {
        Variables::intern(self.interner(), self.original_vars.iter().map(|_| Variable::new()))
    }

    fn add_var(&mut self, var: InferVar<I>) -> usize {
        self.original_vars.iter().position(|v| v == &var).unwrap_or_else(|| {
            let next = self.original_vars.len();
            self.original_vars.push(var);
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
                let index = self.add_var(root);
                Ok(BoundVar::new(DebruijnIdx::ZERO, index).to_ty(self.interner()))
            }
        }
    }
}
