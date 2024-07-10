use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

use super::{FunctionInfo, FunctionTable, SymbolTable, VariableInfo};
use crate::ast::visitor::{Visitor, Walkable};
use crate::ast::{Definition, Function, If, Pattern, Program, While};
use crate::diagnostics::DiagnosticsCell;
use crate::passes::pass::Pass;
use crate::span::Span;

pub struct SymbolTableBuilder<'a> {
    symbol_table: RefCell<SymbolTable>,
    functions: FunctionTable,

    diagnostics: DiagnosticsCell,
    _phantom: PhantomData<&'a ()>,
}

impl SymbolTableBuilder<'_> {
    pub fn new(diagnostics: DiagnosticsCell) -> Self {
        Self {
            symbol_table: RefCell::new(SymbolTable::default()),
            functions: HashMap::new(),
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

    fn define_variable(&self, pattern: &Pattern, span: &Span) {
        if self.symbol_table.borrow().is_shadowing(pattern) {
            self.diagnostics
                .borrow_mut()
                .variable_already_declared(&pattern.name, &pattern.span);
        } else {
            let local_idx = self.symbol_table.borrow().variables.len();
            self.symbol_table.borrow_mut().insert_variable(
                pattern.clone(),
                VariableInfo {
                    uses: 0,
                    local_idx,
                    span: *span,
                },
            );
        }
    }
}

impl<'a> Pass for SymbolTableBuilder<'a> {
    type Input = (&'a Program, DiagnosticsCell);

    type Output = (SymbolTable, FunctionTable);

    fn run((ast, diagnostics): Self::Input) -> Self::Output {
        let mut builder = SymbolTableBuilder::new(diagnostics);
        builder.visit_program(ast);

        let main_pat = Pattern {
            name: "main".to_owned(),
            span: Default::default(),
        };
        match builder.functions.get_mut(&main_pat) {
            Some(main) => {
                main.uses += 1;
            }
            None => {
                builder.diagnostics.borrow_mut().main_not_found();
            }
        }

        (builder.symbol_table.into_inner(), builder.functions)
    }
}

impl<'a> Visitor for SymbolTableBuilder<'a> {
    fn visit_function(&mut self, func: &Function) {
        if self.functions.contains_key(&func.pattern) {
            self.diagnostics
                .borrow_mut()
                .function_already_declared(&func.pattern.name, &func.pattern.span);
        } else {
            let local_idx = self.functions.len();
            self.functions.insert(
                func.pattern.clone(),
                FunctionInfo {
                    uses: 0,
                    local_idx,
                    span: func.span,
                },
            );
        }
        let scope_idx = self.enter_scope();
        func.parameters
            .iter()
            .for_each(|pat| self.define_variable(pat, &pat.span));
        func.body.walk(self);
        self.exit_scope(scope_idx);
    }

    fn visit_if(&mut self, if_expr: &If) {
        if_expr.condition.walk(self);
        let scope_idx = self.enter_scope();
        if_expr.then.walk(self);
        self.exit_scope(scope_idx);
    }

    fn visit_while(&mut self, while_expr: &While) {
        while_expr.condition.walk(self);
        let scope_idx = self.enter_scope();
        while_expr.then.walk(self);
        self.exit_scope(scope_idx);
    }

    fn visit_definition(&mut self, def: &Definition) {
        self.define_variable(&def.pattern, &def.span);
        def.pattern.name.walk(self);
        def.value.walk(self);
    }
}
