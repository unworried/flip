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
use std::cell::RefCell;
use std::rc::Rc;

use self::scope::Scope;
use crate::diagnostics::DiagnosticsCell;
use crate::parser::ast::{Assignment, Ast, Definition, Pattern, Variable};
use crate::parser::P;

mod scope;

pub trait ResolveVisitor {
    fn define(&mut self, resolver: &mut NameResolver);
}

pub type DefinitionId = usize;

#[derive(Debug)]
pub struct DefinitionInfo {
    pub pattern: Pattern,
    pub definition: P<Ast>,
    pub uses: u32,
}

pub struct NameResolver {
    symbol_table: Vec<DefinitionInfo>,
    diagnostics: DiagnosticsCell,
    current_scope: Rc<RefCell<Scope>>,
}

impl NameResolver {
    pub fn new(diagnostics: DiagnosticsCell) -> Self {
        Self {
            symbol_table: Vec::new(),
            diagnostics,
            current_scope: Rc::new(RefCell::new(Scope::new(None))),
        }
    }

    pub fn resolve(mut self, ast: &mut Ast) -> Vec<DefinitionInfo> {
        ast.define(&mut self);
        self.check_references();
        self.symbol_table
    }

    fn check_references(&self) {
        for def in self.symbol_table.iter() {
            if def.uses == 0 {
                self.diagnostics
                    .borrow_mut()
                    .unused_variable(&def.pattern.name, &def.pattern.span);
            }
        }
    }

    pub fn push_definition(&mut self, def: &Definition) {
        let info = DefinitionInfo {
            pattern: def.pattern.clone(),
            definition: def.value.clone(),
            uses: 0,
        };

        self.symbol_table.push(info);
    }

    pub fn push_assignment(&mut self, def: &Assignment) {
        let info = DefinitionInfo {
            pattern: def.pattern.clone(),
            definition: def.value.clone(),
            uses: 0,
        };

        self.symbol_table.push(info);
    }

    pub fn push_scope(&mut self) {
        let new_scope = Rc::new(RefCell::new(Scope::new(Some(Rc::clone(
            &self.current_scope,
        )))));
        self.current_scope = new_scope;
    }

    pub fn pop_scope(&mut self) {
        let parent = self.current_scope.borrow().parent.clone();
        if let Some(parent) = parent {
            self.current_scope = parent;
        }
    }

    pub fn define_symbol(&mut self, name: &str, info: DefinitionInfo) -> DefinitionId {
        let id = self.symbol_table.len();
        self.symbol_table.push(info);
        self.current_scope.borrow_mut().define_symbol(name, id);
        id
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<DefinitionId> {
        self.current_scope.borrow().lookup_symbol(name)
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
            Ast::Definition(def) => {
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
        self.value.define(resolver);
        let info = DefinitionInfo {
            pattern: self.pattern.clone(),
            definition: self.value.clone(),
            uses: 0,
        };
        resolver.define_symbol(&self.pattern.name, info);
    }
}

impl ResolveVisitor for Assignment {
    fn define(&mut self, resolver: &mut NameResolver) {
        self.value.define(resolver);

        if resolver.lookup_symbol(&self.pattern.name).is_none() {
            resolver
                .diagnostics
                .borrow_mut()
                .undeclared_assignment(&self.pattern.name, &self.pattern.span);
            return;
        }

        let info = DefinitionInfo {
            pattern: self.pattern.clone(),
            definition: self.value.clone(),
            uses: 0,
        };
        resolver.define_symbol(&self.pattern.name, info);
    }
}

impl ResolveVisitor for Variable {
    fn define(&mut self, resolver: &mut NameResolver) {
        if let Some(id) = resolver.lookup_symbol(&self.pattern) {
            self.definition = Some(id);
            resolver.symbol_table[id].uses += 1;
        } else {
            resolver
                .diagnostics
                .borrow_mut()
                .undefined_reference(&self.pattern, &self.span);
        }
    }
}
