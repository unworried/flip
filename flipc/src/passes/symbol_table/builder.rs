use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;

use super::{DefinitionType, FunctionInfo, FunctionTable, SymbolInfo, SymbolTable, Type};
use crate::ast::visitor::{Visitor, Walkable};
use crate::ast::{Definition, Function, If, Pattern, Program, While};
use crate::diagnostics::DiagnosticsCell;
use crate::passes::pass::Pass;
use crate::span::Span;

pub struct SymbolTableBuilder<'a> {
    symbol_table: SymbolTable,
    max_scope: usize,
    current_scope: usize,

    functions: FunctionTable,
    argument_idx: usize,

    diagnostics: DiagnosticsCell,
    _phantom: PhantomData<&'a ()>,
}

impl SymbolTableBuilder<'_> {
    pub fn new(diagnostics: DiagnosticsCell) -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            max_scope: 0,
            current_scope: 0,

            functions: HashMap::new(),
            argument_idx: 0,

            diagnostics,
            _phantom: PhantomData,
        }
    }

    fn enter_scope(&mut self) {
        self.symbol_table.insert_scope(self.current_scope);
        self.max_scope += 1;
        self.current_scope = self.max_scope;
    }

    fn exit_scope(&mut self) {
        self.current_scope = self
            .symbol_table
            .lookup_scope(self.current_scope)
            .unwrap()
            .parent
            .unwrap();
    }

    fn define_variable(
        &mut self,
        pattern: &Pattern,
        span: &Span,
        ty: Type,
        def_type: DefinitionType,
    ) {
        if self.symbol_table.is_shadowing(pattern, self.current_scope) {
            self.diagnostics
                .borrow_mut()
                .variable_already_declared(&pattern.name, pattern.span);
        } else {
            let symbol_idx = match def_type {
                DefinitionType::Local => self.symbol_table.scopes[self.current_scope].symbols.len(),
                DefinitionType::Argument => {
                    let idx = self.argument_idx;
                    self.argument_idx += 1;
                    idx
                }
            };

            self.symbol_table.insert_symbol(
                pattern.clone(),
                self.current_scope,
                SymbolInfo {
                    ty,
                    def_type,
                    uses: 0,
                    symbol_idx,
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

        (builder.symbol_table, builder.functions)
    }
}

impl Visitor for SymbolTableBuilder<'_> {
    fn visit_function(&mut self, func: &Function) {
        if self.functions.contains_key(&func.pattern) {
            self.diagnostics
                .borrow_mut()
                .function_already_declared(&func.pattern.name, func.pattern.span);
        } else {
            let local_idx = self.functions.len();
            self.functions.insert(
                func.pattern.clone(),
                FunctionInfo {
                    ty: func.return_type.clone(), // TODO: Fix hack
                    uses: 0,
                    local_idx,
                    span: func.span,
                },
            );
        }
        self.enter_scope();
        func.parameters.iter().for_each(|pat| {
            self.define_variable(pat, &pat.span, Type::Unresolved, DefinitionType::Argument)
        });
        func.body.walk(self);
        self.exit_scope();
    }

    fn visit_if(&mut self, if_expr: &If) {
        if_expr.condition.walk(self);
        self.enter_scope();
        if_expr.then.walk(self);
        self.exit_scope();
    }

    fn visit_while(&mut self, while_expr: &While) {
        while_expr.condition.walk(self);
        self.enter_scope();
        while_expr.then.walk(self);
        self.exit_scope();
    }

    fn visit_definition(&mut self, def: &Definition) {
        self.define_variable(
            &def.pattern,
            &def.span,
            def.value.deref().into(),
            DefinitionType::Local,
        );
        def.pattern.name.walk(self);
        def.value.walk(self);
    }
}
