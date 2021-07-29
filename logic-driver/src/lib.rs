use logic_engine::{RecursiveSolver, Solution};
use logic_ir::*;
use logic_parse::{ast, ParseResult};
use std::sync::Arc;

pub type GenericResult<T> = Result<T, anyhow::Error>;

trait IRDatabase<I> {
    fn interner(&self) -> I;
}

#[salsa::query_group(Logic)]
pub trait LogicDatabase: salsa::Database {}

#[salsa::query_group(Lowering)]
pub trait LoweringDatabase: salsa::Database {
    #[salsa::input]
    fn src(&self) -> Arc<String>;
    fn interner(&self) -> LogicInterner;
    fn ast(&self) -> ParseResult<ast::Program>;
    fn ir(&self) -> ParseResult<Program<logic_ir::LogicInterner>>;
    fn env(&self) -> logic_ir::LogicResult<Environment<LogicInterner>>;
    fn query(&self, unparsed_goal: Arc<String>) -> logic_ir::LogicResult<Solution<LogicInterner>>;
}

#[salsa::database(Lowering, Logic)]
#[derive(Default)]
pub struct Database {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Database {
}

impl Database {
    pub fn new(src: &str) -> Self {
        let mut db = Self::default();
        db.set_src(Arc::new(src.to_owned()));
        db
    }
}

fn ast(db: &dyn LoweringDatabase) -> ParseResult<ast::Program> {
    let src = db.src();
    logic_parse::parse_program(&src)
}

fn ir(db: &dyn LoweringDatabase) -> ParseResult<Program<LogicInterner>> {
    let ast = db.ast()?;
    Ok(logic_ir::lower_ast(&ast).expect("todo error handling"))
}

fn interner(_db: &dyn LoweringDatabase) -> LogicInterner {
    logic_ir::LogicInterner
}

fn env(db: &dyn LoweringDatabase) -> logic_ir::LogicResult<logic_ir::Environment<LogicInterner>> {
    let ir = db.ir().expect("todo proper error handling");
    let env = logic_ir::Environment::new(ir.clauses);
    Ok(env)
}

fn query(
    db: &dyn LoweringDatabase,
    unparsed_goal: Arc<String>,
) -> logic_ir::LogicResult<Solution<LogicInterner>> {
    let env = db.env()?;
    let solver = RecursiveSolver::new(logic_ir::LogicInterner, env);
    let parsed_goal = logic_parse::parse_goal(&unparsed_goal).expect("error handling");
    let goal = logic_ir::lower_goal(&parsed_goal).expect("todo error handling");
    // TODO canonicalize goal properly
    let canonical_goal = Canonical { binders: Variables::empty(LogicInterner), value: goal };
    solver.solve(&canonical_goal)
}

#[cfg(test)]
mod tests;
