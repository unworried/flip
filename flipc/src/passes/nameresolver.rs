//! nameresolver.rs - Defines the variable resolution logic responsible for checking declartions,
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

use super::symbol_table::{FunctionTable, SymbolTable};
use super::Pass;
use crate::ast::visitor::{Visitor, Walkable};
use crate::ast::{Assignment, Call, Function, If, Program, Variable, While};
use crate::diagnostics::DiagnosticsCell;

pub trait ResolveVisitor {
    fn define(&mut self, resolver: &mut NameResolver);
}

pub struct NameResolver<'a> {
    symbol_table: RefCell<SymbolTable>,
    functions: &'a mut FunctionTable,

    diagnostics: DiagnosticsCell,
    scope_idx: usize,
}

impl<'a> NameResolver<'a> {
    pub fn new(
        symbol_table: SymbolTable,
        functions: &'a mut FunctionTable,
        diagnostics: DiagnosticsCell,
    ) -> Self {
        Self {
            symbol_table: RefCell::new(symbol_table),
            functions,
            diagnostics,
            scope_idx: 0,
        }
    }

    fn check_usage(&self) {
        for (pat, def) in self.symbol_table.borrow().symbols.iter() {
            if def.uses == 0 {
                self.diagnostics
                    .borrow_mut()
                    .unused_variable(&pat.name, &pat.span);
            }
        }
    }

    fn check_functions(&mut self) {
        for (pat, func) in self.functions.iter() {
            if func.uses == 0 {
                self.diagnostics
                    .borrow_mut()
                    .unused_function(&pat.name, &pat.span);
            }
        }
    }

    fn enter_scope(&mut self) -> usize {
        let previous_symbol_table = std::mem::take(&mut self.symbol_table);
        self.symbol_table.swap(
            previous_symbol_table
                .borrow()
                .lookup_scope(self.scope_idx)
                .unwrap(),
        );
        self.symbol_table.borrow_mut().parent = Some(Box::new(previous_symbol_table.into_inner()));

        //self.scope_idx
        core::mem::replace(&mut self.scope_idx, 0)
    }

    fn exit_scope(&mut self, index: usize) {
        self.check_usage();

        let previous_symbol_table = *self.symbol_table.borrow_mut().parent.take().unwrap();
        let new_scope = previous_symbol_table.lookup_scope(index).unwrap();
        self.symbol_table.swap(new_scope);
        self.symbol_table = RefCell::new(previous_symbol_table);
        self.scope_idx = index + 1;
    }
}

impl<'a> Pass for NameResolver<'a> {
    type Input = (
        &'a Program,
        SymbolTable,
        &'a mut FunctionTable,
        DiagnosticsCell,
    );

    type Output = SymbolTable;

    fn run((ast, st, funcs, diagnostics): Self::Input) -> Self::Output {
        let mut resolver = NameResolver::new(st, funcs, diagnostics);
        resolver.visit_program(ast);

        resolver.check_functions();

        resolver.check_usage(); // Fix check at root scope. Remove once functions are added.
        resolver.symbol_table.into_inner()
    }
}

// FIXME: Here + Builder, move scope enter/exit to sequence visitor, does not need to be duplicated
impl Visitor for NameResolver<'_> {
    fn visit_function(&mut self, func: &Function) {
        let scope_idx = self.enter_scope();
        func.body.walk(self);
        self.exit_scope(scope_idx);
    }

    fn visit_if(&mut self, if_expr: &If) {
        let scope_idx = self.enter_scope();
        if_expr.condition.walk(self);
        if_expr.then.walk(self);
        self.exit_scope(scope_idx);
    }

    fn visit_while(&mut self, while_expr: &While) {
        let scope_idx = self.enter_scope();
        while_expr.condition.walk(self);
        while_expr.then.walk(self);
        self.exit_scope(scope_idx);
    }

    fn visit_assignment(&mut self, def: &Assignment) {
        if self
            .symbol_table
            .borrow()
            .lookup_symbol(&def.pattern)
            .is_none()
        {
            self.diagnostics
                .borrow_mut()
                .undeclared_assignment(&def.pattern.name, &def.pattern.span);
        }

        def.value.walk(self);
    }

    fn visit_variable(&mut self, var: &Variable) {
        if self.symbol_table.borrow().lookup_symbol(var).is_none() {
            self.diagnostics
                .borrow_mut()
                .undefined_reference(&var.name, &var.span);
        } else {
            let mut st = self.symbol_table.borrow_mut();
            let def = st.lookup_symbol_mut(var).unwrap();
            def.uses += 1;
        }
    }

    fn visit_call(&mut self, call: &Call) {
        if self.functions.get(&call.pattern).is_none() {
            self.diagnostics
                .borrow_mut()
                .undefined_reference(&call.pattern.name, &call.pattern.span);
        } else {
            let func = self.functions.get_mut(&call.pattern).expect("unreachable");
            func.uses += 1;
        }

        call.arguments.iter().for_each(|arg| arg.walk(self));
    }
}
