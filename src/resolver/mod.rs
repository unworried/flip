use crate::{
    cache::Cache,
    parser::{
        ast::{statement::Local, Ast, Ident},
        visitor::Visitor,
    },
};

use self::scope::Scope;

pub mod scope;

pub struct Resolver {
    cache: Cache,
    scope: Scope,
}

impl Resolver {
    pub fn new(cache: Cache) -> Self {
        Self {
            cache,
            scope: Scope::new(),
        }
    }

    pub fn search(&mut self, ast: &Ast) {
        self.visit_ast(ast);
        //println!("{:?}", self.cache.definitions);
    }
}

/*
 * TODO: Need to check declarationations first then assignments. This will allow for more error messages
 * e.g. if a variable is used before it is declared, we can tell the user that the variable is
 * either not defined or is referenced before assignment depending on if the variable is ever
 * declared within the scope.
 */
impl Visitor for Resolver {
    fn visit_declaration(&mut self, local: &Local) {
        let pattern = &local.pattern.0;

        if self.scope.check_variable(pattern) {
            self.cache
                .diagnostics
                .borrow_mut()
                .variable_already_declared(pattern, &local.pattern.1);
            return;
        }

        let id = self.scope.declare_variable(pattern.to_owned());
        self.cache.push_declartion(id, local);
    }

    fn visit_assignment(&mut self, local: &Local) {
        let pattern = &local.pattern.0;

        if !self.scope.check_variable(pattern) {
            self.cache
                .diagnostics
                .borrow_mut()
                .undeclared_variable_assignment(pattern, &local.pattern.1);
            self.visit_local(local);
            return;
        }

        let id = self.scope.declare_variable(pattern.to_owned());
        self.cache.push_definition(id, local);
    }

    fn visit_variable(&mut self, ident: &Ident) {
        let pattern = &ident.0;

        if !self.scope.check_variable(pattern) {
            self.cache
                .diagnostics
                .borrow_mut()
                .reference_before_assignment(pattern, &ident.1);
        }
    }
}
