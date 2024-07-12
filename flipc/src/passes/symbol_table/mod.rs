use std::cell::RefCell;
use std::collections::HashMap;

use crate::ast::Pattern;
use crate::span::Span;

mod builder;

pub use builder::SymbolTableBuilder;

pub type FunctionTable = HashMap<Pattern, FunctionInfo>;

#[derive(Debug)]
pub enum Type {
    Int,
    Char,
    String,
    Bool,
}

#[derive(Debug, Default)]
pub struct SymbolInfo {
    // TODO: Implement Types
    pub ty: Option<Type>,
    pub def_type: DefinitionType,
    pub uses: usize,
    pub symbol_idx: usize,
    span: Span,
}

#[repr(u8)]
#[derive(Debug, Default, Clone, PartialEq)]
pub enum DefinitionType {
    #[default]
    Local,
    Argument,
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
    pub symbols: HashMap<Pattern, SymbolInfo>,
    pub scope_idx: usize,
    scopes: Vec<RefCell<SymbolTable>>,
}

impl SymbolTable {
    pub fn is_shadowing(&self, ident: &Pattern) -> bool {
        if self.symbols.contains_key(ident) {
            true
        } else if let Some(parent) = self.parent.as_ref() {
            parent.is_shadowing(ident)
        } else {
            false
        }
    }

    pub fn lookup_symbol(&self, ident: &Pattern) -> Option<&SymbolInfo> {
        if let Some(var) = self.symbols.get(ident) {
            Some(var)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_symbol(ident)
        } else {
            None
        }
    }

    pub fn update_symbol<F>(&mut self, ident: &Pattern, f: F)
    where
        F: FnOnce(&mut SymbolInfo),
    {
        if let Some(var) = self.symbols.get_mut(ident) {
            f(var);
        } else if let Some(parent) = self.parent.as_mut() {
            parent.update_symbol(ident, f);
        } else {
            unreachable!("Symbol not found in symbol table");
        }
    }

    pub fn local_count(&self) -> usize {
        let mut count = 0;
        count += self
            .symbols
            .iter()
            .filter(|(_, v)| v.def_type == DefinitionType::Local)
            .count();

        for scope in self.scopes.iter() {
            count += scope.borrow().local_count();
        }

        count
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

    pub fn insert_symbol(&mut self, ident: Pattern, variable: SymbolInfo) {
        self.symbols.insert(ident, variable);
    }
}
