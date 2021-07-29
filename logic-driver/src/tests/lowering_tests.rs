use indexed_vec::Idx;
use logic_ir::*;

macro_rules! lower {
    ($program:expr) => {{
        use crate::LoweringDatabase;
        crate::Database::new($program).ir().unwrap()
    }};
}

macro_rules! domain_goal {
    (impl $ty:expr, $trait_ref:expr) => {
        DomainGoal::Holds(Constraint::Implemented(ImplConstraint {
            ty: $ty,
            trait_ref: $trait_ref,
        }))
    };
}

macro_rules! goal {
    (domain $($domain:tt)*) => {
        Goal::intern(LogicInterner, GoalData::DomainGoal(domain_goal!($($domain)*)))
    };
}

macro_rules! program {
    ($($clause:expr)*) => {
        Program::new(LogicInterner, Clauses::intern(LogicInterner, [$($clause)*]))
    }
}

macro_rules! clause_implication {
    // note this macro is backwards w.r.t prolog syntax
    // the implication is forward rather than backwards (due to restrictions of expression macros)
    (for<$n:literal> { $condition:expr => $consequent:expr } ) => {{
        let implication = Implication { consequent: $consequent, condition: $condition };
        let variables = Variables::intern(LogicInterner, (0..$n).map(|_| Variable::new()));
        let binders = Binders::new(variables, implication);
        Clause::intern(LogicInterner, ClauseData::Implies(binders))
    }};
}

macro_rules! ty {
    // using colon to separate as dot separator (like the debug output) will just be treated as a float
    (var $debruijn:literal : $index:literal) => {
        TyKind::Bound(BoundVar { debruijn: DebruijnIdx::new($debruijn), index: $index })
            .intern(LogicInterner)
    };
}

macro_rules! trait_ref {
    ($name:ident) => {
        TraitRef {
            trait_name: Ident::unspanned(stringify!($name)),
            args: Subst::intern(LogicInterner, []),
        }
    };
}

#[test]
fn test_lower_single_forall() {
    let program = r"
        for<X,Y> { X: Trait :- Y: Trait }.
    ";

    let ir_clause = lower!(program).first_clause();

    let expected: logic_ir::Clause<logic_ir::LogicInterner> = clause_implication! {
        for<2> {
            goal!(domain impl ty!(var 0:1), trait_ref!(Trait))
            => domain_goal!(impl ty!(var 0:0), trait_ref!(Trait))
        }
    };

    assert_eq!(ir_clause, expected);
}
