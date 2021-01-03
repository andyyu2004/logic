use crate::*;
use std::fmt::{self, Formatter};

pub trait DebugCtxt<I: Interner> {
    fn dbg_term(&self, term: &Term<I>, fmt: &mut Formatter<'_>) -> fmt::Result;
}

impl DebugCtxt<IRInterner> for IRInterner {
    fn dbg_term(&self, term: &Term<Self>, f: &mut Formatter<'_>) -> fmt::Result {
        match self.term(term) {
            TermData::Atom(atom) => write!(f, "{}", atom),
            TermData::Var(var) => write!(f, "{}", var),
            TermData::Structure(functor, terms) =>
                write!(f, "{} {:?}", functor, util::join_dbg(self.terms(terms), ",")),
        }
    }
}
