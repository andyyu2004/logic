use crate::*;
use logic_parse::ast;

pub fn lower_ast(ast: &ast::Program) -> Program<IRInterner> {
    AstLoweringCtx::default().lower_program(ast)
}

pub fn lower_goal(goal: &ast::Goal) -> Goal<IRInterner> {
    AstLoweringCtx::default().lower_goal(goal)
}

/// lowers ast into ir form, not to be confused with trait lowering
// this is not generic over the interner as we only lower the ast
// which has `term` as its domain goal
struct AstLoweringCtx {
    interner: IRInterner,
}

impl Default for AstLoweringCtx {
    fn default() -> Self {
        Self { interner: IRInterner }
    }
}

type I = IRInterner;

impl AstLoweringCtx {
    pub fn lower_program(&self, program: &ast::Program) -> Program<I> {
        let mut clauses = vec![];
        for item in &program.items {
            match item {
                ast::Item::Clause(clause) => clauses.push(self.lower_clause(clause)),
            }
        }

        Program { clauses: Clauses::intern(self.interner, clauses), interner: self.interner }
    }

    pub fn lower_goal(&self, goal: &ast::Goal) -> Goal<I> {
        let goal_data = match goal {
            ast::Goal::DomainGoal(domain_goal) =>
                GoalData::DomainGoal(self.lower_domain_goal(domain_goal)),
            ast::Goal::Implies(clause, goal) =>
                GoalData::Implies(self.lower_clause(clause), self.lower_goal(goal)),
            ast::Goal::And(lhs, rhs) => GoalData::And(self.lower_goal(lhs), self.lower_goal(rhs)),
            ast::Goal::Or(lhs, rhs) => GoalData::Or(self.lower_goal(lhs), self.lower_goal(rhs)),
        };
        Goal::intern(self.interner, goal_data)
    }

    pub fn lower_domain_goal(&self, domain_goal: &ast::DomainGoal) -> DomainGoal<I> {
        todo!()
    }

    pub fn lower_goals(&self, goals: &[ast::Goal]) -> Goals<I> {
        Goals::intern(self.interner, goals.into_iter().map(|goal| self.lower_goal(goal)))
    }

    pub fn lower_tys<'a>(&self, tys: &[ast::Ty]) -> Tys<I> {
        Tys::intern(self.interner, tys.into_iter().map(|ty| self.lower_ty(ty)))
    }

    pub fn lower_ty(&self, ty: &ast::Ty) -> Ty<I> {
        let kind = match ty {
            ast::Ty::Structure(functor, terms) => todo!(), // TyData::Structure(*functor, self.lower_tys(terms)),
        };
        // Ty::new(self.interner, self.interner.intern_tys(kind))
    }

    pub fn lower_clause(&self, clause: &ast::Clause) -> Clause<I> {
        todo!()
        // let lowered_clause = match clause {
        // ast::Clause::Forall(var, clause) => todo!(),
        // ast::Clause::Horn(consequent, goals) =>
        // ClauseData::Horn(self.lower_term(consequent), self.lower_goals(goals)),
        // };
        // Clause::new(self.interner, self.interner.intern_clause(lowered_clause))
    }
}
