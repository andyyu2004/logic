use ena::unify::UnificationTable;
use ir::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InferenceVar<I: Interner> {
    idx: u32,
    phantom: std::marker::PhantomData<I>,
}

impl<I: Interner> InferenceVar<I> {
    pub fn new(idx: u32) -> Self {
        Self { idx, phantom: std::marker::PhantomData }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InferenceValue<I: Interner> {
    Term(Term<I>),
}

impl<I: Interner> ena::unify::UnifyKey for InferenceVar<I> {
    type Value = InferenceValue<I>;

    fn index(&self) -> u32 {
        self.idx
    }

    fn from_index(idx: u32) -> Self {
        Self::new(idx)
    }

    fn tag() -> &'static str {
        "InferenceVar"
    }
}

impl<I: Interner> ena::unify::EqUnifyValue for InferenceValue<I> {
}

#[derive(Clone)]
pub struct InferCtxt<I: Interner> {
    tables: ena::unify::InPlaceUnificationTable<InferenceVar<I>>,
    vars: Vec<InferenceVar<I>>,
}

impl<I: Interner> InferCtxt<I> {
    pub fn unify(&self, t: &TermData<I>, u: &TermData<I>) -> Substs<I> {
        debug_assert_ne!(t, u);
        match (t, u) {
            (TermData::Var(x), _) => todo!(),
            (_, TermData::Var(y)) => todo!(),
            (TermData::Structure(f, xs), TermData::Structure(g, ys)) if f == g => todo!(),
            _ => panic!("failure"),
        }
    }
}
