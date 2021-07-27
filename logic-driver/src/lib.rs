use logic_engine::{RecursiveSolver, Solution};
use logic_ir::{Environment, LogicInterner, LogicResult, Program};
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
    fn ir(&self) -> ParseResult<Program<LogicInterner>>;
    fn env(&self) -> LogicResult<Environment<LogicInterner>>;
    fn query(&self, unparsed_goal: Arc<String>) -> LogicResult<Solution<LogicInterner>>;
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
    LogicInterner
}

fn env(db: &dyn LoweringDatabase) -> LogicResult<Environment<LogicInterner>> {
    let ir = db.ir().expect("todo proper error handling");
    let env = Environment::new(ir.clauses);
    Ok(env)
}

fn query(
    db: &dyn LoweringDatabase,
    unparsed_goal: Arc<String>,
) -> LogicResult<Solution<LogicInterner>> {
    let env = db.env()?;
    let solver = RecursiveSolver::new(LogicInterner, env);
    let parsed_goal = logic_parse::parse_goal(&unparsed_goal).expect("error handling");
    let goal = logic_ir::lower_goal(&parsed_goal).expect("todo error handling");
    solver.solve(&goal)
}
