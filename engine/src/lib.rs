mod db;
mod unify;

pub use db::Database;

use ir::*;

pub type LogicResult<T> = Result<T, LogicError>;

impl From<String> for LogicError {
    fn from(s: String) -> Self {
        LogicError(s)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LogicError(String);

pub trait Solver<I: Interner> {
    fn solve(&mut self, db: &Database, env: Environment, goal: &Goal<I>) -> Solution<I> {
        todo!()
    }
}

pub struct RecursiveSolver<I: Interner> {
    interner: I,
}

impl<I: Interner> Solver<I> for RecursiveSolver<I> {
    fn solve(&mut self, db: &Database, env: Environment, goal: &Goal<I>) -> Solution<I> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    clauses: InternedClauses<IRInterner>,
}

impl Environment {
    pub fn new(clauses: <IRInterner as Interner>::InternedClauses) -> Self {
        Self { clauses }
    }
}

pub enum Solution<I: Interner> {
    Unique(Substs<I>),
}
