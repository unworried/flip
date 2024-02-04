use crate::{
    diagnostics::DiagnosticsCell,
    parser::{
        ast::{statement::Local, Ast, Expr, Ident},
        visitor::{Visitor, Walkable},
    },
};

use self::scope::Scope;

mod idxvec;
pub mod scope;

pub struct Resolver {
    scopes: Vec<Scope>,
    diagnostics: DiagnosticsCell,
}

impl Resolver {
    pub fn new(scopes: Vec<Scope>, diagnostics: DiagnosticsCell) -> Self {
        Self {
            scopes,
            diagnostics,
        }
    }

    pub fn resolve(&mut self, ast: &Ast) {
        self.visit_ast(ast);
    }
}

/*
 * TODO: Need to check declarationations first then assignments. This will allow for more error messages
 * e.g. if a variable is used before it is declared, we can tell the user that the variable is
 * either not defined or is referenced before assignment depending on if the variable is ever
 * declared within the scope.
 */
impl Visitor for Resolver {
    fn visit_local(&mut self, local: &Local) {
        let scope = self.scopes.last_mut().unwrap();
        let pattern = &local.pattern.0;

        if scope.variables.iter().any(|v| v == pattern) {
            self.diagnostics
                .borrow_mut()
                .variable_already_declared(pattern, &local.pattern.1);
            return;
        }

        scope.declare_variable(pattern.to_owned());
    }

    fn visit_assignment(&mut self, ident: &Ident, expr: &Expr) {
        let scope = self.scopes.last_mut().unwrap();
        let pattern = &ident.0;

        if scope.variables.iter().all(|v| v != pattern) {
            self.diagnostics
                .borrow_mut()
                .undeclared_variable_assignment(pattern, &ident.1);
            expr.walk(self);
            return;
        }

        scope.declare_variable(pattern.to_owned());
    }

    fn visit_variable(&mut self, ident: &Ident) {
        let scope = self.scopes.last_mut().unwrap();
        let pattern = &ident.0;

        if scope.variables.iter().all(|v| v != pattern) {
            self.diagnostics
                .borrow_mut()
                .reference_before_assignment(pattern, &ident.1);
        }
    }
}
