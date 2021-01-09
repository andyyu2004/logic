use logic_ir::*;

pub struct SubstsFolder<I> {
    interner: I,
}

impl<I: Interner> SubstsFolder<I> {
    pub fn apply_substs(&mut self, substs: Substs<I>, term: Term<I>) -> Term<I> {
        let data = self.interner.term_data(&term.interned);
        match data {
            TermData::Var(x) => todo!(),
            TermData::Infer(infer) => todo!(),
            TermData::Atom(x) => term.clone(),
            TermData::Structure(f, terms) => Term::intern(self.interner, todo!()),
        }
    }
}
