use std::cell::RefCell;
use std::marker::PhantomData;

use crate::ast::visitor::{Visitor, Walkable};
use crate::ast::{Ast, Definition, If, While};
use crate::diagnostics::DiagnosticsCell;
use crate::passes::pass::Pass;

use super::{SymbolTable, VariableInfo};

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
}

impl<'a> Pass for SymbolTableBuilder<'a> {
    type Input = (&'a Ast, DiagnosticsCell);

    type Output = SymbolTable;

    fn run((ast, diagnostics): Self::Input) -> Self::Output {
        let mut builder = SymbolTableBuilder::new(diagnostics);
        builder.visit_ast(ast);
        builder.symbol_table.into_inner()
    }
}

fn enter_scope(builder: &mut SymbolTableBuilder) -> usize {
    let scope_idx = builder.symbol_table.borrow_mut().insert_scope();
    let previous_symbol_table = std::mem::take(&mut builder.symbol_table);
    builder.symbol_table.swap(
        previous_symbol_table
            .borrow()
            .lookup_scope(scope_idx)
            .unwrap(),
    );
    builder.symbol_table.borrow_mut().parent = Some(Box::new(previous_symbol_table.into_inner()));

    scope_idx
}

fn exit_scope(builder: &mut SymbolTableBuilder, index: usize) {
    let previous_symbol_table = *builder.symbol_table.borrow_mut().parent.take().unwrap();
    let new_scope = previous_symbol_table.lookup_scope(index).unwrap();
    builder.symbol_table.swap(new_scope);
    builder.symbol_table = RefCell::new(previous_symbol_table);
}

impl<'a> Visitor for SymbolTableBuilder<'a> {
    fn visit_if(&mut self, if_expr: &If) {
        let scope_idx = enter_scope(self);
        if_expr.condition.walk(self);
        if_expr.then.walk(self);
        exit_scope(self, scope_idx);
    }

    fn visit_while(&mut self, while_expr: &While) {
        let scope_idx = enter_scope(self);
        while_expr.condition.walk(self);
        while_expr.then.walk(self);
        exit_scope(self, scope_idx);
    }

    fn visit_definition(&mut self, def: &Definition) {
        if self.symbol_table.borrow().is_shadowing(&def.pattern) {
            self.diagnostics
                .borrow_mut()
                .symbol_already_declared(&def.pattern.name, &def.pattern.span);
        } else {
            self.symbol_table.borrow_mut().insert_variable(
                def.pattern.clone(),
                VariableInfo {
                    uses: 0,
                    span: def.span,
                },
            );
        }

        def.pattern.name.walk(self);
        def.value.walk(self);
    }
}
