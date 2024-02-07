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
use crate::diagnostics::DiagnosticsCell;
use crate::parser::ast::{Ast, Definition, Variable};

use self::scope::Scope;

//pub mod evaluator;
mod scope;

pub trait ResolveVisitor {
    fn define(&mut self, resolver: &mut NameResolver);
}

pub type DefinitionId = usize;

#[derive(Debug)]
pub struct DefinitionInfo {
    pub definition: Definition,
    pub uses: u32,
}

pub struct NameResolver {
    diagnostics: DiagnosticsCell,
    definitions: Vec<DefinitionInfo>,
    scopes: Vec<Scope>,
}

impl NameResolver {
    pub fn new(diagnostics: DiagnosticsCell) -> Self {
        Self {
            diagnostics,
            definitions: Vec::new(),
            scopes: Vec::new(),
        }
    }

    pub fn resolve(&mut self, ast: &mut Ast) {
        ast.define(self);
        self.check_references();
    }

    fn check_references(&self) {
        for def in self.definitions.iter() {
            if def.uses == 0 {
                // TODO: Fix so only pattern is underlined
                self.diagnostics
                    .borrow_mut()
                    .unused_variable(&def.definition.pattern.name, &def.definition.pattern.span);
            }
        }
    }

    pub fn push_definition(&mut self, def: &Definition) {
        let info = DefinitionInfo {
            definition: def.clone(),
            uses: 0,
        };

        self.definitions.push(info);
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}

impl ResolveVisitor for Ast {
    fn define(&mut self, resolver: &mut NameResolver) {
        match self {
            Ast::Sequence(seq) => {
                resolver.push_scope();
                for node in seq.statements.iter_mut() {
                    node.define(resolver);
                }
                resolver.pop_scope();
            }
            Ast::Let(def) => {
                def.define(resolver);
            }
            Ast::Assignment(def) => {
                def.define(resolver);
            }
            Ast::Variable(var) => {
                var.define(resolver);
            }
            Ast::Binary(bin) => {
                bin.left.define(resolver);
                bin.right.define(resolver);
            }
            Ast::Unary(un) => {
                un.operand.define(resolver);
            }
            Ast::If(if_expr) => {
                if_expr.condition.define(resolver);
                if_expr.then.define(resolver);
            }
            Ast::While(while_expr) => {
                while_expr.condition.define(resolver);
                while_expr.then.define(resolver);
            }
            Ast::Literal(_) => {}
            Ast::Error => {}
        }
    }
}

impl ResolveVisitor for Definition {
    fn define(&mut self, resolver: &mut NameResolver) {
        let id = resolver.definitions.len();
        self.id = Some(id);
        resolver.push_definition(self);
        resolver
            .scopes
            .last_mut()
            .expect("callstack should not be empty")
            .define_symbol(&self.pattern.name.clone(), id);

        self.value.define(resolver);
    }
}

impl ResolveVisitor for Variable {
    fn define(&mut self, resolver: &mut NameResolver) {
        // Default var definition is None, no ast passes have been made therefore it can be assumed this
        // is still true.
        assert!(self.definition.is_none());
        assert!(!resolver.scopes.is_empty());

        for scope in resolver.scopes.iter().rev() {
            if let Some(id) = scope.lookup_symbol(&self.pattern) {
                self.definition = Some(*id);

                let def = &mut resolver
                    .definitions
                    .get_mut(*id)
                    .expect("scope lookup already checked");
                def.uses += 1;
                return;
            }
        }

        resolver
            .diagnostics
            .borrow_mut()
            .undefined_reference(&self.pattern, &self.span);
    }
}
