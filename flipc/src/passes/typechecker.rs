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
    symbol_table: &'a mut SymbolTable,
    max_scope: usize,
    current_scope: usize,

    functions: &'a mut FunctionTable,

    diagnostics: DiagnosticsCell,
}

impl<'a> Pass for TypeChecker<'a> {
    type Input = (
        &'a Program,
        &'a mut SymbolTable,
        &'a mut FunctionTable,
        DiagnosticsCell,
    );

    type Output = ();

    fn run((program, symbol_table, functions, diagnostics): Self::Input) -> Self::Output {
        let mut type_checker = Self::new(symbol_table, functions, diagnostics);
        type_checker.visit_program(program);
    }
}

impl<'a> TypeChecker<'a> {
    pub fn new(
        symbol_table: &'a mut SymbolTable,
        functions: &'a mut FunctionTable,
        diagnostics: DiagnosticsCell,
    ) -> Self {
        Self {
            symbol_table,
            max_scope: 0,
            current_scope: 0,
            functions,
            diagnostics,
        }
    }

    fn enter_scope(&mut self) {
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
}

// FIXME: Here + Builder, move scope enter/exit to sequence visitor, does not need to be duplicated
impl Visitor for TypeChecker<'_> {
    fn visit_function(&mut self, func: &Function) {
        self.enter_scope();
        func.body.walk(self);
        self.exit_scope();
    }

    fn visit_if(&mut self, if_expr: &If) {
        self.enter_scope();
        if_expr.condition.walk(self);
        if_expr.then.walk(self);
        self.exit_scope();
    }

    fn visit_while(&mut self, while_expr: &While) {
        self.enter_scope();
        while_expr.condition.walk(self);
        while_expr.then.walk(self);
        self.exit_scope();
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
            .update_symbol(&def.pattern, self.current_scope, |def| {
                def.ty = Some(def_type);
            });
    }

    fn visit_assignment(&mut self, def: &Assignment) {}

    fn visit_variable(&mut self, var: &Variable) {}

    fn visit_call(&mut self, call: &Call) {}
}
