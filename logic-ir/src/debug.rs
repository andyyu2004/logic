use crate::*;
use std::fmt::{self, Formatter};

pub trait DebugCtxt<I: Interner> {
    fn dbg_ty(&self, term: &Ty<I>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_tys(&self, tys: &Tys<I>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_goal(&self, goal: &Goal<I>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_goals(&self, goals: &Goals<I>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_clause(&self, clause: &Clause<I>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_clauses(&self, clauses: &Clauses<I>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_subst(&self, subst: &Subst<I>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_generic_arg(&self, generic_arg: &GenericArg<I>, fmt: &mut Formatter) -> fmt::Result;
}

impl<I: Interner> DebugCtxt<I> for I {
    fn dbg_ty(&self, ty: &Ty<Self>, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self.ty_data(ty))
    }

    fn dbg_tys(&self, tys: &Tys<Self>, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", util::join_dbg(tys.as_slice(), ","))
    }

    fn dbg_goal(&self, goal: &Goal<Self>, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self.goal_data(goal))
    }

    fn dbg_goals(&self, goals: &Goals<Self>, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self.goals(goals))
    }

    fn dbg_clause(&self, clause: &Clause<Self>, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self.clause_data(clause))
    }

    fn dbg_subst(&self, subst: &Subst<Self>, fmt: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }

    fn dbg_clauses(&self, clauses: &Clauses<Self>, fmt: &mut Formatter<'_>) -> fmt::Result {
        for clause in clauses.as_slice() {
            self.dbg_clause(clause, fmt)?;
            writeln!(fmt)?;
        }
        Ok(())
    }

    fn dbg_generic_arg(&self, generic_arg: &GenericArg<I>, fmt: &mut Formatter) -> fmt::Result {
        todo!()
    }
}
