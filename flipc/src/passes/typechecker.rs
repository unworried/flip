use std::cell::RefCell;
use std::ops::Deref;

use crate::ast::visitor::{Visitor, Walkable};
use crate::ast::{
    Assignment, Call, Definition, Function, If, Literal, LiteralKind, Program, Variable, While,
};
use crate::diagnostics::DiagnosticsCell;
use crate::{Ast, Pass};

use super::symbol_table::{FunctionTable, Type};
use super::SymbolTable;

pub struct TypeChecker<'a> {
    symbol_table: RefCell<SymbolTable>,
    functions: &'a mut FunctionTable,

    diagnostics: DiagnosticsCell,
    scope_idx: usize,
}

impl<'a> Pass for TypeChecker<'a> {
    type Input = (
        &'a Program,
        SymbolTable,
        &'a mut FunctionTable,
        DiagnosticsCell,
    );

    type Output = SymbolTable;

    fn run((program, symbol_table, functions, diagnostics): Self::Input) -> Self::Output {
        let mut type_checker = Self::new(symbol_table, functions, diagnostics);
        type_checker.visit_program(program);

        type_checker.symbol_table.into_inner()
    }
}

impl<'a> TypeChecker<'a> {
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
        let previous_symbol_table = *self.symbol_table.borrow_mut().parent.take().unwrap();
        let new_scope = previous_symbol_table.lookup_scope(index).unwrap();
        self.symbol_table.swap(new_scope);
        self.symbol_table = RefCell::new(previous_symbol_table);
        self.scope_idx = index + 1;
    }
}

// FIXME: Here + Builder, move scope enter/exit to sequence visitor, does not need to be duplicated
impl Visitor for TypeChecker<'_> {
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
        let def_type = match def.value.deref() {
            Ast::Literal(Literal { kind, .. }) => match kind {
                LiteralKind::Int(_) => Type::Int,
                LiteralKind::Char(_) => Type::Char,
                LiteralKind::String(_) => Type::String,
            },
            //Ast::Variable(_) => {}
            //Ast::Call(_) => {}
            //Ast::Binary(_) => {}
            //Ast::Unary(_) => {}
            _ => unreachable!("{:#?}", def.value),
        };

        self.symbol_table
            .borrow_mut()
            .update_symbol(&def.pattern, |def| {
                def.ty = Some(def_type);
            });
    }

    fn visit_assignment(&mut self, def: &Assignment) {}

    fn visit_variable(&mut self, var: &Variable) {}

    fn visit_call(&mut self, call: &Call) {}
}
