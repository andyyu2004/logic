mod db;

pub use db::Database;

use ir::*;

pub type LogicResult<T> = Result<T, LogicError>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LogicError(String);

pub trait Solver<I: Interner> {
    fn solve(&mut self, db: &Database, env: Environment, goal: &Goal<I>) -> Solution {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    clauses: InternedClauses<IRInterner>,
}

impl Environment {
    pub fn new() -> Self {
        Self { clauses: vec![] }
    }
}

pub enum Solution {}
