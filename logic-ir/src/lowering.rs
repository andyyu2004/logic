use std::collections::HashMap;

use crate::*;
use logic_parse::ast;

pub fn lower_ast(ast: &ast::Program) -> LoweringResult<Program<LogicInterner>> {
    AstLoweringCtx::default().lower_program(ast)
}

pub fn lower_goal(goal: &ast::Goal) -> LoweringResult<Goal<LogicInterner>> {
    AstLoweringCtx::default().lower_goal(goal)
}

/// lowers ast into ir form, not to be confused with trait lowering
// this is not generic over the interner as we only lower the ast
// which has `term` as its domain goal
struct AstLoweringCtx {
    interner: LogicInterner,
    env: Env,
}

#[derive(Debug, Default)]
struct Env {
    variables: HashMap<Symbol, BoundVar>,
}

pub type LoweringResult<T> = Result<T, LoweringError>;

#[derive(Debug)]
pub enum LoweringError {}

impl Env {
    fn lookup_type(&self, symbol: &Symbol) -> LoweringResult<Option<Ty<LogicInterner>>> {
        let ty = match self.variables.get(symbol) {
            Some(&bound) => Some(TyKind::Bound(bound).intern(LogicInterner)),
            None => None,
        };
        Ok(ty)
    }
}

impl Default for AstLoweringCtx {
    fn default() -> Self {
        Self { interner: LogicInterner, env: Default::default() }
    }
}

impl AstLoweringCtx {
    pub fn lower_program(
        &mut self,
        program: &ast::Program,
    ) -> LoweringResult<Program<LogicInterner>> {
        let mut clauses = vec![];
        for item in &program.items {
            match item {
                ast::Item::Clause(clause) => clauses.push(self.lower_clause(clause)?),
            }
        }

        Ok(Program { clauses: Clauses::intern(self.interner, clauses), interner: self.interner })
    }

    pub fn lower_goal(&mut self, goal: &ast::Goal) -> LoweringResult<Goal<LogicInterner>> {
        let goal_data = match goal {
            ast::Goal::DomainGoal(domain_goal) =>
                GoalData::DomainGoal(self.lower_domain_goal(domain_goal)?),
            ast::Goal::Implies(clause, goal) =>
                GoalData::Implies(self.lower_clause(clause)?, self.lower_goal(goal)?),
            ast::Goal::And(lhs, rhs) => GoalData::And(self.lower_goal(lhs)?, self.lower_goal(rhs)?),
            ast::Goal::Or(lhs, rhs) => GoalData::Or(self.lower_goal(lhs)?, self.lower_goal(rhs)?),
        };
        Ok(Goal::intern(self.interner, goal_data))
    }

    pub fn lower_domain_goal(
        &mut self,
        domain_goal: &ast::DomainGoal,
    ) -> LoweringResult<DomainGoal<LogicInterner>> {
        match domain_goal {
            ast::DomainGoal::Holds(constraint) =>
                self.lower_constraint(constraint).map(DomainGoal::Holds),
        }
    }

    pub fn lower_constraint(
        &mut self,
        constraint: &ast::Constraint,
    ) -> LoweringResult<Constraint<LogicInterner>> {
        let lowered = match constraint {
            ast::Constraint::Implemented(impl_constraint) =>
                Constraint::Implemented(ImplConstraint {
                    ty: self.lower_ty(&impl_constraint.ty)?,
                    trait_ref: self.lower_trait_ref(&impl_constraint.trait_ref)?,
                }),
        };
        Ok(lowered)
    }

    pub fn lower_trait_ref(
        &mut self,
        trait_ref: &ast::TraitRef,
    ) -> LoweringResult<TraitRef<LogicInterner>> {
        Ok(TraitRef {
            trait_name: trait_ref.trait_name.clone(),
            args: self.lower_tys(&trait_ref.args)?,
        })
    }

    pub fn _lower_goals(&mut self, goals: &[ast::Goal]) -> LoweringResult<Goals<LogicInterner>> {
        Goals::try_intern(self.interner, goals.into_iter().map(|goal| self.lower_goal(goal)))
    }

    pub fn lower_tys<'a>(&mut self, tys: &[ast::Ty]) -> LoweringResult<Subst<LogicInterner>> {
        Subst::try_intern(self.interner, tys.into_iter().map(|ty| self.lower_ty(ty)))
    }

    pub fn lower_ty(&mut self, ty: &ast::Ty) -> LoweringResult<Ty<LogicInterner>> {
        let kind = match ty {
            ast::Ty::Structure(functor, tys) => match &tys[..] {
                // if no arguments it might be referencing a type by name
                &[] => match self.env.lookup_type(&functor.symbol)? {
                    Some(ty) => return Ok(ty),
                    None => TyKind::Structure(functor.clone(), Subst::empty(self.interner)),
                },
                _ => TyKind::Structure(functor.clone(), self.lower_tys(tys)?),
            },
        };

        Ok(kind.intern(self.interner))
    }

    pub fn lower_clause(&mut self, clause: &ast::Clause) -> LoweringResult<Clause<LogicInterner>> {
        let clause_data = match clause {
            // lower known domain goals into an implication with a trivially true condition
            ast::Clause::DomainGoal(domain_goal) => ClauseData::Implies(Binders::empty(
                self.interner,
                Implication {
                    consequent: self.lower_domain_goal(domain_goal)?,
                    condition: Goal::intern(self.interner, GoalData::True),
                },
            )),
            ast::Clause::Implies(implication) =>
                ClauseData::Implies(self.lower_implication(implication)?),
            ast::Clause::And(_, _) => todo!(),
        };
        Ok(Clause::intern(self.interner, clause_data))
    }

    pub fn lower_implication(
        &mut self,
        implication: &ast::Implication,
    ) -> LoweringResult<Binders<Implication<LogicInterner>>> {
        let interner = self.interner;

        // TODO deBruijn shifting etc, just checking there is no shifting to be done for now
        assert!(self.env.variables.is_empty());

        let variables = implication
            .vars
            .iter()
            .enumerate()
            .map(|(i, var)| {
                self.env
                    .variables
                    .insert(var.ident.symbol.clone(), BoundVar::new(DebruijnIdx::ZERO, i));
                Variable::new()
            })
            .collect::<Vec<_>>();

        let consequent = self.lower_domain_goal(&implication.consequent)?;
        let condition = self.lower_goal(&implication.condition)?;

        let value = Implication { consequent, condition };
        Ok(Binders::new(Variables::intern(interner, variables), value))
    }
}

#[cfg(test)]
mod tests;
