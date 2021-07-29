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
            ast::Goal::Exists(vars, subgoal) => GoalData::Quantified(
                Quantifier::Exists,
                self.enter_binders(vars, |this| this.lower_goal(subgoal))?,
            ),
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

    pub fn enter_binders<R>(
        &mut self,
        binder_slice: &[ast::Var],
        f: impl FnOnce(&mut Self) -> LoweringResult<R>,
    ) -> LoweringResult<Binders<R>>
    where
        R: HasInterner<Interner = LogicInterner>,
    {
        let expected_len = self.env.variables.len() + binder_slice.len();

        let binders = binder_slice
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, var)| (var.ident.symbol, BoundVar::new(DebruijnIdx::ZERO, i)));

        let variables = self
            .env
            .variables
            .iter()
            .map(|(name, bound)| (name.clone(), bound.shifted_in()))
            .chain(binders)
            .collect::<HashMap<_, _>>();

        assert_eq!(variables.len(), expected_len, "found shadowed parameter names");

        let value = f(&mut Self { env: Env { variables }, interner: self.interner })?;
        Ok(Binders {
            binders: Variables::intern(
                LogicInterner,
                (0..binder_slice.len()).map(|_| Variable::new()),
            ),
            quantified: value,
        })
    }

    pub fn lower_implication(
        &mut self,
        implication: &ast::Implication,
    ) -> LoweringResult<Binders<Implication<LogicInterner>>> {
        self.enter_binders(&implication.vars, |ctxt| {
            let consequent = ctxt.lower_domain_goal(&implication.consequent)?;
            let condition = ctxt.lower_goal(&implication.condition)?;
            Ok(Implication { consequent, condition })
        })
    }
}

#[cfg(test)]
mod tests;
