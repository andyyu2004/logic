use logic_ir::*;

macro_rules! query {
    ($src:ident:  $goal:tt) => {{
        use logic_driver::LoweringDatabase;
        logic_driver::Database::new($src).query(std::sync::Arc::new($goal.to_owned())).unwrap()
    }};
}

#[test]
fn test_solve_goal_by_simple_implication() {
    let program = r"
    Option<i32>: Clone :- Option<i32>: Copy.
    Option<i32>: Copy.
    ";
    let solution = query!(program: "Option<i32>: Clone");
    assert_eq!(solution.into_subst(), subst![]);
}

#[test]
fn test_solve_simple_existence_goal() {
    let program = r"i32: Copy.";
    let solution = query!(program: "exists<T> { T: Copy }");
    assert_eq!(solution.into_subst(), subst![ty!(i32)]);
}

#[test]
fn test_solve_goal_by_instantiating_forall_clause() {
    let program = r"
    for<T> { Vec<T>: Clone :- T: Clone }.
    i32: Clone.
    ";
    let solution = query!(program: "Vec<i32>: Clone");
    assert_eq!(solution.into_subst(), subst![]);
}

#[tracing_test::traced_test]
#[test]
fn test_solve_multistep_goal() {
    let program = r"
    i32: Eq.
    for<T> { Vec<T>: Eq :- T: Eq }.
    for<T> { T: PartialEq :- T: Eq }.
    ";
    let solution = query!(program: "exists<T> { Vec<T> : PartialEq }");
    assert_eq!(solution.into_subst(), subst![]);
}

// http://rust-lang.github.io/chalk/book/recursive/stack.html
#[tracing_test::traced_test]
#[test]
fn test_solve_goal_chalk_example() {
    let program = r"
    for<T> { Vec<T>: A :- T: B }.
    u32: B.
    ";
    let solution = query!(program: "exists<T> { Vec<T>: A }");
    assert_eq!(solution.into_subst(), subst![ty!(u32)]);
}
