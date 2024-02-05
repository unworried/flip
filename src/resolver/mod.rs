use crate::{
    cache::{Cache, DefinitionId, DefinitionKind},
    parser::{
        ast::{statement::Local, Ast, Ident},
        visitor::Visitor,
    },
};

use self::scope::Scope;

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
    }

    /*
     * TODO: Cleanup this code. This is terrible
     */
    fn check_references(&mut self) {
        self.cache
            .definitions
            .borrow()
            .iter()
            .for_each(|(_, info)| {
                if info.kind == DefinitionKind::Reference {
                    let pattern = &info.pattern;
                    match self.scope.variables.borrow().get(pattern) {
                        Some(parent_id) => {
                            let parent_span = self
                                .cache
                                .definitions
                                .borrow()
                                .get(parent_id)
                                .unwrap()
                                .span
                                .clone();
                            if parent_span > info.span {
                                self.cache
                                    .diagnostics
                                    .borrow_mut()
                                    .reference_before_assignment(pattern, &info.span);
                            }
                        }

                        None => {
                            self.cache
                                .diagnostics
                                .borrow_mut()
                                .undeclared_reference(pattern, &info.span);
                        }
                    }
                }
            });
    }

    fn push_parent(&mut self, child_ident: &str) -> Result<(), ()> {
        match self.scope.variables.borrow().get(child_ident) {
            Some(id) => {
                self.cache.push_parent(id, &DefinitionId(self.scope.count));
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
                .diagnostics
                .borrow_mut()
                .symbol_already_declared(pattern, &local.pattern.1);
            self.visit_local(local);
            return;
        }

        let id = self.scope.declare_variable(pattern.to_owned());

        self.cache.push_declartion(id, local);
        self.visit_local(local);
    }

    fn visit_assignment(&mut self, local: &Local) {
        self.cache
            .push_assignment(DefinitionId(self.scope.count), local);

        if self.push_parent(&local.pattern.0).is_err() {
            self.cache
                .diagnostics
                .borrow_mut()
                .undeclared_assignment(&local.pattern.0, &local.pattern.1);
        };

        self.visit_local(local);
    }

    fn visit_variable(&mut self, ident: &Ident) {
        let id = DefinitionId(self.scope.count);
        self.scope.count += 1;
        self.cache.push_reference(id, ident);
    }
}
