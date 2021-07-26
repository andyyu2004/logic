use indexed_vec::Idx;
// use indexed_vec::Idx;
use logic_ir::*;
use std::cell::RefCell;
use std::fmt::{self, Debug, Formatter};

pub enum UnificationError {
    // TODO
    Failed,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InferVar<I: Interner> {
    idx: InferenceIdx,
    phantom: std::marker::PhantomData<I>,
}

impl<I: Interner> InferVar<I> {
    pub fn new(idx: InferenceIdx) -> Self {
        Self { idx, phantom: std::marker::PhantomData }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InferenceValue<I: Interner> {
    Bound(Ty<I>),
    Unbound,
}

impl<I: Interner> Debug for InferVar<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "?{:?}", self.idx)
    }
}

impl<I: Interner> ena::unify::UnifyValue for InferenceValue<I> {
    type Error = ena::unify::NoError;

    /// Given two values, produce a new value that combines them.
    /// If that is not possible, produce an error.
    fn unify_values(x: &Self, y: &Self) -> Result<Self, Self::Error> {
        Ok(match (x, y) {
            (Self::Bound(..), Self::Bound(..)) => panic!("unifying two known values"),
            (Self::Bound(..), Self::Unbound) => x.clone(),
            (Self::Unbound, Self::Bound(..)) => y.clone(),
            (Self::Unbound, Self::Unbound) => Self::Unbound,
        })
    }
}

impl<I: Interner> ena::unify::UnifyKey for InferVar<I> {
    type Value = InferenceValue<I>;

    fn index(&self) -> u32 {
        self.idx.index() as u32
    }

    fn from_index(idx: u32) -> Self {
        Self::new(InferenceIdx::new(idx as usize))
    }

    fn tag() -> &'static str {
        "InferenceVar"
    }
}

pub struct InferCtxt<I: Interner> {
    interner: I,
    inner: RefCell<InferCtxtInner<I>>,
}

pub struct InferCtxtInner<I: Interner> {
    tables: ena::unify::InPlaceUnificationTable<InferVar<I>>,
    vars: Vec<InferVar<I>>,
}

impl<I: Interner> Default for InferCtxtInner<I> {
    fn default() -> Self {
        Self { tables: Default::default(), vars: Default::default() }
    }
}

impl<I: Interner> InferCtxt<I> {
    pub fn new(interner: I) -> Self {
        Self { interner, inner: Default::default() }
    }
}
