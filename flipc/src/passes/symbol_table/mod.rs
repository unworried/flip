use std::collections::HashMap;
use std::fmt;

use crate::ast::{Literal, LiteralKind, Pattern};
use crate::span::Span;
use crate::Ast;

mod builder;

pub use builder::SymbolTableBuilder;

pub type FunctionTable = HashMap<Pattern, FunctionInfo>;

#[derive(Debug, PartialEq)]
pub enum Type {
    Unresolved,
    Int,
    Char,
    String,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Unresolved => write!(f, "unresolved"),
            Type::Int => write!(f, "int"),
            Type::Char => write!(f, "char"),
            Type::String => write!(f, "string"),
        }
    }
}

impl From<&Ast> for Type {
    fn from(value: &Ast) -> Self {
        match value {
            Ast::Literal(Literal { kind, .. }) => match kind {
                LiteralKind::Int(_) => Type::Int,
                LiteralKind::Char(_) => Type::Char,
                LiteralKind::String(_) => Type::String,
            },
            //Ast::Variable(_) => {}
            //Ast::Call(_) => {}
            //Ast::Binary(_) => {}
            //Ast::Unary(_) => {}
            _ => unreachable!("{:#?}", value),
        }
    }
}

#[derive(Debug)]
pub struct SymbolInfo {
    pub ty: Type,
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

#[derive(Debug)]
pub struct SymbolTable {
    pub scopes: Vec<Scope>,
}

#[derive(Debug, Default)]
pub struct Scope {
    pub parent: Option<usize>,
    pub symbols: HashMap<Pattern, SymbolInfo>,
}

impl Scope {
    pub fn new(parent: usize) -> Self {
        Self {
            parent: Some(parent),
            symbols: HashMap::new(),
        }
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::default()],
        }
    }

    pub fn is_shadowing(&self, ident: &Pattern, scope_idx: usize) -> bool {
        if self.scopes[scope_idx].symbols.contains_key(ident) {
            true
        } else if let Some(parent) = self.scopes[scope_idx].parent {
            self.is_shadowing(ident, parent)
        } else {
            false
        }
    }

    pub fn lookup_symbol(&self, ident: &Pattern, scope_idx: usize) -> Option<&SymbolInfo> {
        if let Some(var) = self.scopes[scope_idx].symbols.get(ident) {
            Some(var)
        } else if let Some(parent) = self.scopes[scope_idx].parent {
            self.lookup_symbol(ident, parent)
        } else {
            None
        }
    }

    pub fn update_symbol<F>(&mut self, ident: &Pattern, scope_idx: usize, f: F)
    where
        F: FnOnce(&mut SymbolInfo),
    {
        if let Some(var) = self.scopes[scope_idx].symbols.get_mut(ident) {
            f(var);
        } else if let Some(parent) = self.scopes[scope_idx].parent {
            self.update_symbol(ident, parent, f);
        } else {
            unreachable!("Symbol not found in symbol table");
        }
    }

    pub fn local_count(&self) -> usize {
        let mut count = 0;
        for scope in self.scopes.iter() {
            count += scope
                .symbols
                .iter()
                .filter(|(_, v)| v.def_type == DefinitionType::Local)
                .count();
        }

        count
    }

    pub fn lookup_scope(&self, idx: usize) -> Option<&Scope> {
        self.scopes.get(idx)
    }

    pub fn insert_scope(&mut self, parent: usize) {
        self.scopes.push(Scope::new(parent));
    }

    pub fn insert_symbol(&mut self, ident: Pattern, scope_idx: usize, variable: SymbolInfo) {
        self.scopes[scope_idx].symbols.insert(ident, variable);
    }
}
