use crate::*;
use ir::{IRInterner, Program};
use parse::{ast, ParseResult};
use std::sync::Arc;

trait IRDatabase<I> {
    fn interner(&self) -> I;
}

#[salsa::query_group(Logic)]
trait LogicDatabase: salsa::Database {}

#[salsa::query_group(Lowering)]
trait LoweringDatabase: salsa::Database {
    #[salsa::input]
    fn src(&self) -> Arc<String>;
    fn ast(&self) -> ParseResult<ast::Program>;
    fn ir(&self) -> ParseResult<Program<IRInterner>>;
    fn env(&self) -> LogicResult<Environment>;
}

#[salsa::database(Lowering, Logic)]
#[derive(Default)]
pub struct Database {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Database {
}

impl Database {
    pub fn new(src: String) -> Self {
        let mut db = Self::default();
        db.set_src(Arc::new(src));
        db
    }
}

fn ast(db: &dyn LoweringDatabase) -> ParseResult<ast::Program> {
    let src = db.src();
    parse::parse_program(&src)
}

fn ir(db: &dyn LoweringDatabase) -> ParseResult<Program<IRInterner>> {
    let ast = db.ast()?;
    Ok(ir::lower_ast(IRInterner, &ast))
}

fn env(db: &dyn LoweringDatabase) -> LogicResult<Environment> {
    let env = Environment::new();
    Ok(env)
}
