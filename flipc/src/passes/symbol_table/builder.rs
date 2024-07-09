use std::cell::RefCell;
use std::marker::PhantomData;

use super::{SymbolTable, VariableInfo};
use crate::ast::visitor::{Visitor, Walkable};
use crate::ast::{Definition, Function, If, Program, While};
use crate::diagnostics::DiagnosticsCell;
use crate::passes::pass::Pass;

pub struct SymbolTableBuilder<'a> {
    symbol_table: RefCell<SymbolTable>,
    diagnostics: DiagnosticsCell,
    _phantom: PhantomData<&'a ()>,
}

impl SymbolTableBuilder<'_> {
    pub fn new(diagnostics: DiagnosticsCell) -> Self {
        Self {
            symbol_table: RefCell::new(SymbolTable::default()),
            diagnostics,
            _phantom: PhantomData,
        }
    }

    fn enter_scope(&mut self) -> usize {
        let scope_idx = self.symbol_table.borrow_mut().insert_scope();
        let previous_symbol_table = std::mem::take(&mut self.symbol_table);
        self.symbol_table.swap(
            previous_symbol_table
                .borrow()
                .lookup_scope(scope_idx)
                .unwrap(),
        );
        self.symbol_table.borrow_mut().parent = Some(Box::new(previous_symbol_table.into_inner()));

        scope_idx
    }

    fn exit_scope(&mut self, index: usize) {
        let previous_symbol_table = *self.symbol_table.borrow_mut().parent.take().unwrap();
        let new_scope = previous_symbol_table.lookup_scope(index).unwrap();
        self.symbol_table.swap(new_scope);
        self.symbol_table = RefCell::new(previous_symbol_table);
    }
}

impl<'a> Pass for SymbolTableBuilder<'a> {
    type Input = (&'a Program, DiagnosticsCell);

    type Output = SymbolTable;

    fn run((ast, diagnostics): Self::Input) -> Self::Output {
        let mut builder = SymbolTableBuilder::new(diagnostics);
        builder.visit_program(ast);
        builder.symbol_table.into_inner()
    }
}

impl<'a> Visitor for SymbolTableBuilder<'a> {
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

    fn visit_definition(&mut self, def: &Definition) {
        if self.symbol_table.borrow().is_shadowing(&def.pattern) {
            self.diagnostics
                .borrow_mut()
                .symbol_already_declared(&def.pattern.name, &def.pattern.span);
        } else {
            let local_idx = self.symbol_table.borrow().variables.len();
            self.symbol_table.borrow_mut().insert_variable(
                def.pattern.clone(),
                VariableInfo {
                    uses: 0,
                    local_idx,
                    span: def.span,
                },
            );
        }

        def.pattern.name.walk(self);
        def.value.walk(self);
    }
}
