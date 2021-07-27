use logic_ir::{LogicInterner, Subst};

macro_rules! query {
    ($src:ident:  $goal:tt) => {{
        use logic_driver::LoweringDatabase;
        logic_driver::Database::new($src).query(std::sync::Arc::new($goal.to_owned())).unwrap()
    }};
}

#[test]
fn test_solve_simple_implication() {
    let program = r"
    Option<i32>: Clone :- Option<i32>: Copy.
    Option<i32>: Copy.
    ";
    let solution = query!(program: "Option<i32>: Clone");
    assert_eq!(solution.into_unique(), Subst::empty(LogicInterner));
}

#[test]
fn test_solve() {
    let program = r"
    forall<T> { Vec<T>: Clone :- T: Clone }.
    i32: Clone.
    ";
    let solution = query!(program: "Vec<i32>: Clone");
    dbg!(solution);
}
