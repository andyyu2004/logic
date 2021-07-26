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

impl AstLoweringCtx {
    pub fn lower_program(&self, program: &ast::Program) -> Program<IRInterner> {
        let mut clauses = vec![];
        for item in &program.items {
            match item {
                ast::Item::Clause(clause) => clauses.push(self.lower_clause(clause)),
            }
        }

        Program { clauses: Clauses::intern(self.interner, clauses), interner: self.interner }
    }

    pub fn lower_goal(&self, goal: &ast::Goal) -> Goal<IRInterner> {
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

    pub fn lower_domain_goal(&self, domain_goal: &ast::DomainGoal) -> DomainGoal<IRInterner> {
        match domain_goal {
            ast::DomainGoal::Holds(constraint) =>
                DomainGoal::Holds(self.lower_constraint(constraint)),
        }
    }

    pub fn lower_constraint(&self, constraint: &ast::Constraint) -> Constraint<IRInterner> {
        match constraint {
            ast::Constraint::Implemented(impl_constraint) =>
                Constraint::Implemented(ImplConstraint {
                    ty: self.lower_ty(&impl_constraint.ty),
                    trait_ref: self.lower_trait_ref(&impl_constraint.trait_ref),
                }),
        }
    }

    pub fn lower_trait_ref(&self, trait_ref: &ast::TraitRef) -> TraitRef<IRInterner> {
        TraitRef { trait_name: trait_ref.trait_name.clone(), args: self.lower_tys(&trait_ref.args) }
    }

    pub fn lower_goals(&self, goals: &[ast::Goal]) -> Goals<IRInterner> {
        Goals::intern(self.interner, goals.into_iter().map(|goal| self.lower_goal(goal)))
    }

    pub fn lower_tys<'a>(&self, tys: &[ast::Ty]) -> Tys<IRInterner> {
        Tys::intern(self.interner, tys.into_iter().map(|ty| self.lower_ty(ty)))
    }

    pub fn lower_ty(&self, ty: &ast::Ty) -> Ty<IRInterner> {
        let kind = match ty {
            ast::Ty::Structure(functor, tys) =>
                TyKind::Structure(functor.clone(), self.lower_tys(tys)),
        };
        kind.intern(self.interner)
    }

    pub fn lower_clause(&self, clause: &ast::Clause) -> Clause<IRInterner> {
        let clause_data = match clause {
            // lower known domain goals into an implication with a trivially true condition
            ast::Clause::DomainGoal(domain_goal) => ClauseData::Implies(
                self.lower_domain_goal(domain_goal),
                Goal::intern(self.interner, GoalData::True),
            ),
            ast::Clause::Implies(domain_goal, goal) =>
                ClauseData::Implies(self.lower_domain_goal(domain_goal), self.lower_goal(goal)),
            ast::Clause::ForAll(_, _) => todo!(),
            ast::Clause::And(_, _) => todo!(),
        };
        Clause::intern(self.interner, clause_data)
    }
}
