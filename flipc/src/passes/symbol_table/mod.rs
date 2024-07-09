use std::cell::RefCell;
use std::collections::HashMap;

use crate::ast::Pattern;
use crate::span::Span;

mod builder;

pub use builder::SymbolTableBuilder;

#[derive(Debug, Default)]
pub struct VariableInfo {
    // TODO: Implement Types
    // type_: Type,
    pub uses: usize,
    pub local_idx: usize,
    span: Span,
}

#[derive(Debug, Default)]
pub struct FunctionInfo {
    pub uses: usize,
    pub local_idx: usize,
    span: Span,
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    pub parent: Option<Box<SymbolTable>>,
    pub variables: HashMap<Pattern, VariableInfo>,
    pub functions: HashMap<Pattern, FunctionInfo>,
    pub scope_idx: usize,
    scopes: Vec<RefCell<SymbolTable>>,
}

impl SymbolTable {
    pub fn is_shadowing_var(&self, ident: &Pattern) -> bool {
        if self.variables.contains_key(ident) {
            true
        } else if let Some(parent) = self.parent.as_ref() {
            parent.is_shadowing_var(ident)
        } else {
            false
        }
    }

    pub fn is_shadowing_func(&self, ident: &Pattern) -> bool {
        if self.functions.contains_key(ident) {
            true
        } else if let Some(parent) = self.parent.as_ref() {
            parent.is_shadowing_func(ident)
        } else {
            false
        }
    }

    pub fn lookup_variable(&self, ident: &Pattern) -> Option<&VariableInfo> {
        if let Some(var) = self.variables.get(ident) {
            Some(var)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_variable(ident)
        } else {
            None
        }
    }

    pub fn lookup_variable_mut(&mut self, ident: &Pattern) -> Option<&mut VariableInfo> {
        if let Some(var) = self.variables.get_mut(ident) {
            Some(var)
        } else if let Some(parent) = self.parent.as_mut() {
            parent.lookup_variable_mut(ident)
        } else {
            None
        }
    }

    pub fn lookup_scope(&self, idx: usize) -> Option<&RefCell<SymbolTable>> {
        self.scopes.get(idx)
    }

    pub fn insert_scope(&mut self) -> usize {
        self.scopes.push(Default::default());
        let idx = self.scope_idx;
        self.scope_idx += 1;
        idx
    }

    pub fn insert_variable(&mut self, ident: Pattern, variable: VariableInfo) {
        self.variables.insert(ident, variable);
    }

    pub fn insert_function(&mut self, ident: Pattern, function: FunctionInfo) {
        self.functions.insert(ident, function);
    }
}
