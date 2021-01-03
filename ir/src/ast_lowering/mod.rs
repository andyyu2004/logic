use crate::*;
use parse::ast;

pub fn lower_ast<I: Interner>(interner: I, ast: &ast::Program) -> Program<I> {
    AstLoweringCtx { interner }.lower_program(ast)
}

/// lowers ast into ir form, not to be confused with trait lowering
struct AstLoweringCtx<I: Interner> {
    interner: I,
}

impl<I: Interner> AstLoweringCtx<I> {
    pub fn lower_program(&self, program: &ast::Program) -> Program<I> {
        let mut clauses = vec![];
        for item in &program.items {
            match item {
                ast::Item::Clause(clause) => clauses.push(self.lower_clause(clause)),
            }
        }

        Program { clauses: Clauses(self.interner.intern_clauses(clauses)), interner: self.interner }
    }

    pub fn lower_goal(&self, goal: &ast::Goal) -> Goal<I> {
        let lowered_goal = match goal {
            ast::Goal::Term(term) => GoalData::Term(self.lower_term(term)),
            ast::Goal::Implies(clause, goal) => todo!(),
            ast::Goal::And(lhs, rhs) => GoalData::And(self.lower_goal(lhs), self.lower_goal(rhs)),
            ast::Goal::Or(lhs, rhs) => GoalData::Or(self.lower_goal(lhs), self.lower_goal(rhs)),
        };
        Goal(self.interner.intern_goal(lowered_goal))
    }

    pub fn lower_terms<'a>(&self, terms: &[ast::Term]) -> Terms<I> {
        Terms(self.interner.intern_terms(terms.into_iter().map(|term| self.lower_term(term))))
    }

    pub fn lower_goals(&self, goals: &[ast::Goal]) -> Goals<I> {
        Goals(self.interner.intern_goals(goals.into_iter().map(|goal| self.lower_goal(goal))))
    }

    pub fn lower_term(&self, term: &ast::Term) -> Term<I> {
        let term = match term {
            &ast::Term::Atom(atom) => TermData::Atom(atom),
            &ast::Term::Var(var) => TermData::Var(var),
            ast::Term::Structure(functor, terms) =>
                TermData::Structure(*functor, self.lower_terms(terms)),
        };
        Term(self.interner.intern_term(term))
    }

    pub fn lower_clause(&self, clause: &ast::Clause) -> Clause<I> {
        let lowered_clause = match clause {
            ast::Clause::Forall(var, clause) => todo!(),
            ast::Clause::Horn(consequent, goals) =>
                ClauseData::Horn(self.lower_term(consequent), self.lower_goals(goals)),
        };
        Clause(self.interner.intern_clause(lowered_clause))
    }
}
