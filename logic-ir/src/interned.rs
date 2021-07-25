//! macros for creating wrappers around the interned associated types
use crate::*;
use std::fmt::{self, Debug, Formatter};

macro_rules! interned {
    ($get_data:ident => $data:ident, $intern:ident => $ty:ident, $interned:ident, $dbg_method:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $ty<I: Interner> {
            pub interner: I,
            pub interned: I::$interned,
        }

        impl<I: Interner> $ty<I> {
            pub fn new(interner: I, interned: I::$interned) -> Self {
                Self { interner, interned }
            }

            pub fn data(&self) -> &$data<I> {
                self.interner.$get_data(self)
            }

            pub fn intern(interner: I, data: $data<I>) -> Self {
                Self { interner, interned: interner.$intern(data) }
            }
        }

        impl<I: Interner> std::ops::Deref for $ty<I> {
            type Target = I::$interned;

            fn deref(&self) -> &Self::Target {
                &self.interned
            }
        }

        impl<I: Interner> std::ops::DerefMut for $ty<I> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.interned
            }
        }

        impl<I: Interner> Debug for $ty<I> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                self.interner.$dbg_method(self, f)
            }
        }
    };
}

macro_rules! interned_slice {
    ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident, $dbg_method:ident) => {
        /// List of interned elements.
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $seq<I: Interner> {
            pub interner: I,
            pub interned: I::$interned,
        }

        impl<I: Interner> $seq<I> {
            pub fn intern(interner: I, iter: impl IntoIterator<Item = $elem>) -> Self {
                Self { interner, interned: interner.$intern(iter) }
            }

            pub fn interned(&self) -> &I::$interned {
                &self.interned
            }

            pub fn as_slice(&self) -> &[$elem] {
                self.interner.$data(&self.interned)
            }

            pub fn at(&self, index: usize) -> &$elem {
                &self.as_slice()[index]
            }

            pub fn is_empty(&self) -> bool {
                self.as_slice().is_empty()
            }

            pub fn iter(&self) -> std::slice::Iter<'_, $elem> {
                self.as_slice().iter()
            }

            pub fn len(&self) -> usize {
                self.as_slice().len()
            }
        }

        impl<I: Interner> std::ops::Deref for $seq<I> {
            type Target = I::$interned;

            fn deref(&self) -> &Self::Target {
                &self.interned
            }
        }

        impl<I: Interner> std::ops::DerefMut for $seq<I> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.interned
            }
        }

        impl<I: Interner> std::fmt::Debug for $seq<I> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                self.interner.$dbg_method(self, f)
            }
        }
    };
}

interned!(goal_data => GoalData, intern_goal => Goal, InternedGoal, dbg_goal);
interned!(clause_data => ClauseData, intern_clause => Clause, InternedClause, dbg_clause);
interned!(ty_data => TyData, intern_ty => Ty, InternedTy, dbg_ty);
interned!(generic_arg_data => GenericArgData, intern_generic_arg => GenericArg, InternedGenericArg, dbg_generic_arg);

interned_slice!(
    Subst,
    subst_data => GenericArg<I>,
    intern_subst => InternedSubst,
    dbg_subst
);

interned_slice!(
    Tys,
    tys => Ty<I>,
    intern_tys => InternedTys,
    dbg_tys
);

interned_slice!(
    Clauses,
    clauses => Clause<I>,
    intern_clauses => InternedClauses,
    dbg_clauses
);

interned_slice!(
    Goals,
    goals => Goal<I>,
    intern_goals => InternedGoals,
    dbg_goals
);
