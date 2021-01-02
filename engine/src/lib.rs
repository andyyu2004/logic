use ir::*;

pub trait Solver<I: Interner> {
    fn solve(&mut self, db: Database, goal: &Goal<I>) -> Solution {
        todo!()
    }
}

pub struct Database {}

pub enum Solution {}
