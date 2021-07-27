use crate::*;
use std::fmt::{self, Formatter};

pub trait DebugCtxt: Interner {
    fn dbg_ty(&self, term: &Ty<Self>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_tys(&self, tys: &Subst<Self>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_goal(&self, goal: &Goal<Self>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_goals(&self, goals: &Goals<Self>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_clause(&self, clause: &Clause<Self>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_clauses(&self, clauses: &Clauses<Self>, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn dbg_subst(&self, subst: &Subst<Self>, fmt: &mut Formatter<'_>) -> fmt::Result;
}

impl<I: Interner> DebugCtxt for I {
    fn dbg_ty(&self, ty: &Ty<Self>, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self.ty_data(ty))
    }

    fn dbg_tys(&self, tys: &Subst<Self>, fmt: &mut Formatter<'_>) -> fmt::Result {
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
        write!(fmt, "[{:?}]", util::join_dbg(subst.as_slice(), ","))
    }

    fn dbg_clauses(&self, clauses: &Clauses<Self>, fmt: &mut Formatter<'_>) -> fmt::Result {
        for clause in clauses.as_slice() {
            self.dbg_clause(clause, fmt)?;
            writeln!(fmt)?;
        }
        Ok(())
    }
}
