#[macro_export]
macro_rules! lower_goal {
    ($program:expr) => {{
        let goal_ast = logic_parse::parse_goal($program).unwrap();
        logic_ir::lower_goal(&goal_ast).unwrap()
    }};
}

#[macro_export]
macro_rules! domain_goal {
    (impl $ty:expr, $trait_ref:expr) => {
        DomainGoal::Holds(Constraint::Implemented(ImplConstraint {
            ty: $ty,
            trait_ref: $trait_ref,
        }))
    };
}

#[macro_export]
macro_rules! bound {
    ($n:literal, $bound:expr) => {{
        let variables = Variables::intern(LogicInterner, (0..$n).map(|_| Variable::new()));
        Binders::new(variables, $bound)
    }};
}

#[macro_export]
macro_rules! goal {
    (domain $($domain:tt)*) => {
        Goal::intern(LogicInterner, GoalData::DomainGoal(domain_goal!($($domain)*)))
    };
    (exists <$n:literal> $($goal:tt)*) => {{
        let bound = bound!($n, goal!($($goal)*));
        Goal::intern(LogicInterner, GoalData::Quantified(Quantifier::Exists, bound))
    }};
}

#[macro_export]
macro_rules! program {
    ($($clause:expr)*) => {
        Program::new(LogicInterner, Clauses::intern(LogicInterner, [$($clause)*]))
    }
}

#[macro_export]
macro_rules! clause_implication {
    // note this macro is backwards w.r.t prolog syntax
    // the implication is forward rather than backwards (due to restrictions of expression macros)
    (for<$n:literal> { $condition:expr => $consequent:expr } ) => {{
        let implication = Implication { consequent: $consequent, condition: $condition };
        let binders = bound!($n, implication);
        Clause::intern(LogicInterner, ClauseData::Implies(binders))
    }};
}

#[macro_export]
macro_rules! ty {
    // using colon to separate as dot separator (like the debug output) will just be treated as a float
    (var $debruijn:literal : $index:literal) => {
        $crate::TyKind::Bound(BoundVar { debruijn: DebruijnIdx::new($debruijn), index: $index })
            .intern(LogicInterner)
    };
    ($name:ty) => {
        $crate::TyKind::Structure(
            $crate::Ident::unspanned(stringify!($name)),
            $crate::Subst::empty($crate::LogicInterner),
        )
        .intern(LogicInterner)
    };
    ($name:ident) => {
        TyKind::Structure(Ident::unspanned($name), Subst::empty(LogicInterner))
            .intern(LogicInterner)
    };
}

#[macro_export]
macro_rules! trait_ref {
    ($name:ident) => {
        TraitRef {
            trait_name: Ident::unspanned(stringify!($name)),
            args: Subst::intern(LogicInterner, []),
        }
    };
}
