use crate::*;
use parse::ast;

struct AstLoweringCtx<I: Interner> {
    interner: I,
}

impl<I: Interner> AstLoweringCtx<I> {
    pub fn lower_goal(&self, goal: &ast::Goal) -> InternedGoal<I> {
        let lowered_goal = match goal {
            ast::Goal::Term(term) => Goal::Term(self.lower_term(term)),
            ast::Goal::And(lhs, rhs) => Goal::And(self.lower_goal(lhs), self.lower_goal(rhs)),
            ast::Goal::Or(lhs, rhs) => Goal::Or(self.lower_goal(lhs), self.lower_goal(rhs)),
        };
        Interned::new(self.interner.intern_goal(lowered_goal))
    }

    pub fn lower_terms<'a>(&self, terms: &[ast::Term]) -> InternedTerms<I> {
        Interned::new(
            self.interner.intern_terms(terms.into_iter().map(|term| self.lower_term(term))),
        )
    }

    pub fn lower_goals(&self, goals: &[ast::Goal]) -> InternedGoals<I> {
        Interned::new(
            self.interner.intern_goals(goals.into_iter().map(|goal| self.lower_goal(goal))),
        )
    }

    pub fn lower_term(&self, term: &ast::Term) -> InternedTerm<I> {
        let term = match term {
            &ast::Term::Atom(atom) => Term::Atom(atom),
            &ast::Term::Var(var) => Term::Var(var),
            ast::Term::Structure(atom, terms) => Term::Structure(*atom, self.lower_terms(terms)),
        };
        Interned::new(self.interner.intern_term(term))
    }

    pub fn lower_clause(&self, clause: &ast::Clause) -> InternedClause<I> {
        let lowered_clause = match clause {
            ast::Clause::Horn(consequent, goals) =>
                Clause::Horn(self.lower_term(consequent), self.lower_goals(goals)),
        };
        Interned::new(self.interner.intern_clause(lowered_clause))
    }
}
