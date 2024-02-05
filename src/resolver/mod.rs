use crate::{
    cache::{Cache, DefinitionKind},
    parser::{
        ast::{statement::Local, Ast, Expr, ExprKind, Ident, Literal},
        visitor::Visitor,
    },
};

use self::{evaluator::evaluate_expression, scope::Scope};

mod evaluator;
mod scope;

pub struct Resolver<'a> {
    cache: &'a Cache,
    scope: Scope,
}

impl<'a> Resolver<'a> {
    pub fn new(cache: &'a Cache) -> Self {
        Self {
            cache,
            scope: Scope::new(),
        }
    }

    pub fn resolve(&mut self, ast: &Ast) {
        self.visit_ast(ast);
        self.check_references();
        self.evaluate_parents();
    }

    fn evaluate_parents(&mut self) {
        for (_, info) in self.cache.definitions.borrow_mut().iter_mut() {
            let expr = match &info.expr {
                Some(expr) => {
                    if let ExprKind::Literal(_) = expr.kind {
                        continue;
                    }
                    expr
                }
                None => continue,
            };

            match evaluate_expression(expr) {
                Some(value) => {
                    info.expr = Some(Expr {
                        kind: ExprKind::Literal(Literal::Integer(value)),
                        span: expr.span.clone(),
                    });
                }
                None => {
                    continue;
                }
            }
        }
    }

    /*
     * TODO: Cleanup this code. This is terrible
     */
    fn check_references(&mut self) {
        for (_, info) in self.cache.definitions.borrow().iter() {
            if info.kind != DefinitionKind::Reference {
                continue;
            }

            let pattern = &info.pattern;
            match self.scope.variables.borrow().get(pattern) {
                Some(parent_id) => {
                    if let Some(parent_info) = self.cache.definitions.borrow().get(parent_id) {
                        if parent_info.span > info.span {
                            self.cache
                                .diagnostics()
                                .reference_before_assignment(pattern, &info.span);
                        }
                    }
                }

                None => {
                    self.cache
                        .diagnostics()
                        .undeclared_reference(pattern, &info.span);
                }
            }
        }
    }

    fn push_child(&mut self, child_ident: &str) -> Result<(), ()> {
        match self.scope.variables.borrow().get(child_ident) {
            Some(id) => {
                self.cache.push_child(id, &self.scope.count);
                Ok(())
            }

            None => Err(()),
        }
    }
}

impl Visitor for Resolver<'_> {
    fn visit_declaration(&mut self, local: &Local) {
        let pattern = &local.pattern.0;

        if self.scope.check_variable(pattern) {
            self.cache
                .diagnostics()
                .symbol_already_declared(pattern, &local.pattern.1);
            self.visit_local(local);
            return;
        }

        let id = self.scope.declare_variable(pattern.to_owned());

        self.cache.push_declartion(id, local);
        self.visit_local(local);
    }

    fn visit_assignment(&mut self, local: &Local) {
        self.cache.push_assignment(self.scope.count, local);

        if self.push_child(&local.pattern.0).is_err() {
            self.cache
                .diagnostics()
                .undeclared_assignment(&local.pattern.0, &local.pattern.1);
        };
        self.scope.count += 1;

        self.visit_local(local);
    }

    fn visit_variable(&mut self, ident: &Ident) {
        let id = self.scope.count;
        self.scope.count += 1;
        self.cache.push_reference(id, ident);
    }
}
