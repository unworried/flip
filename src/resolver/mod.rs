//! resolver/mod.rs - Defines the variable resolution logic responsible for checking declartions,
//! assignments and references. Linear Binary Equations are evaluated and replaced with their
//! constant result. Variable assignments are linked in a chain starting from the root variable to
//! the leaf.
//!
//! The goal of the resolver is to ensure that all variables are declared before they are used, and
//! that all assignments are valid.
//!
//! The resolver is implemented as a visitor pattern, where the resolver visits the AST and builds
//! a definition map.
//!
//! The follow diagnostics can be returned from this module:
//! - symbol_already_declared: The symbol has already been declared in the current scope.
//! - undeclared_assignment: The symbol has not been declared before it was assigned.
//! - undeclared_reference: The symbol has not been declared before it was referenced.
//! - reference_before_assignment: The symbol was referenced before it was declared.
use self::scope::Scope;
use crate::cache::{Cache, DefinitionKind};
use crate::parser::ast::statement::Local;
use crate::parser::ast::{Ast, Ident};
use crate::parser::visitor::Visitor;

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
        //self.evaluate_parents();
    }

    /*fn evaluate_parents(&self) {
        for (_, info) in self.cache.definitions.iter_mut() {
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
    }*/

    // TODO: Cleanup this code. This is terrible
    fn check_references(&self) {
        for (_, info) in self.cache.definitions.borrow().iter() {
            if info.kind != DefinitionKind::Reference {
                continue;
            }

            let pattern = &info.pattern;
            match self.scope.variables.get(pattern) {
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
        match self.scope.variables.get(child_ident) {
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
