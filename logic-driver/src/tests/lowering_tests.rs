use indexed_vec::Idx;
use logic_ir::*;

macro_rules! lower {
    ($program:expr) => {{
        use crate::LoweringDatabase;
        crate::Database::new($program).ir().unwrap()
    }};
}

#[test]
fn test_lower_single_forall() {
    let program = r"
        for<X,Y> { X: Trait :- Y: Trait }.
    ";

    let ir_clause = lower!(program).first_clause();

    let expected = clause_implication! {
        for<2> {
            goal!(domain impl ty!(var 0:1), trait_ref!(Trait))
            => domain_goal!(impl ty!(var 0:0), trait_ref!(Trait))
        }
    };

    assert_eq!(ir_clause, expected);
}

#[test]
fn test_lower_existence_goal() {
    let program = r"
        for<X> {
            X: Trait :- exists<Y> { Y: Trait }
        }.
    ";
    let ir_clause = lower!(program).first_clause();

    let expected = clause_implication! {
        for<1> {
            goal!(exists<1> domain impl ty!(var 0:0), trait_ref!(Trait))
            => domain_goal!(impl ty!(var 0:0), trait_ref!(Trait))
        }
    };

    assert_eq!(ir_clause, expected);
}

#[test]
fn test_lower_nested_quantified_goal() {
    let goal = r"
        exists<X> {
            exists<Y> { X: Trait }
        }
    ";

    // checks the debruijn reference to the outer variable has been shifted correctly
    let expected = goal!(exists<1> exists<1> domain
        impl ty!(var 1:0), trait_ref!(Trait)
    );
    assert_eq!(expected, lower_goal!(goal));
}
