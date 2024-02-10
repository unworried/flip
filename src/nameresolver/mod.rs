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
use crate::diagnostics::DiagnosticsCell;
use crate::parser::ast::{Assignment, Ast, Definition, Pattern, Variable};
use crate::parser::P;

//pub mod evaluator;
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
    scopes: Vec<Scope>,
}

impl NameResolver {
    pub fn new(diagnostics: DiagnosticsCell) -> Self {
        Self {
            symbol_table: Vec::new(),
            diagnostics,
            scopes: Vec::new(),
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
                // TODO: Fix so only pattern is underlined
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
        let id = resolver.symbol_table.len();
        resolver.push_definition(self);
        resolver
            .scopes
            .last_mut()
            .expect("callstack should not be empty")
            .define_symbol(&self.pattern.name.clone(), id);
    }
}

impl ResolveVisitor for Assignment {
    fn define(&mut self, resolver: &mut NameResolver) {
        self.value.define(resolver);

        if resolver
            .scopes
            .last()
            .expect("callstack should not be empty")
            .lookup_symbol(&self.pattern.name)
            .is_none()
        {
            resolver
                .diagnostics
                .borrow_mut()
                .undeclared_assignment(&self.pattern.name, &self.pattern.span);
            return;
        }

        let id = resolver.symbol_table.len();
        resolver.push_assignment(self);
        resolver
            .scopes
            .last_mut()
            .expect("callstack should not be empty")
            .define_symbol(&self.pattern.name.clone(), id);
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
                    .symbol_table
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
