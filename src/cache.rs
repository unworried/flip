//! cache.rs - Module storing the global cache for the compiler frontend. The cache is used to
//! store the symbols table, links and the diagnostics stack.
use std::cell::{RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::diagnostics::{DiagnosticBag, DiagnosticsCell};
use crate::parser::ast::statement::Local;
use crate::parser::ast::{Expr, Ident};
use crate::span::Span;

pub type DefinitionId = usize;

#[derive(Debug, Clone)]
pub struct DefinitionInfo {
    pub pattern: String,
    pub kind: DefinitionKind,
    pub span: Span,
    pub expr: Option<Expr>,
    pub child: Option<DefinitionId>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefinitionKind {
    Declaration,
    Assignment,
    Reference,
}

// TODO: Add diagnostics only in cache (Centralize) then cache can be made on large Cell not
// individuals
pub struct Cache {
    pub definitions: RefCell<HashMap<DefinitionId, DefinitionInfo>>,
    diagnostics: DiagnosticsCell,
}

impl Cache {
    pub fn new(diagnostics: DiagnosticsCell) -> Cache {
        Self {
            definitions: RefCell::new(HashMap::new()),
            diagnostics,
        }
    }
}

impl Cache {
    pub fn get(&self, id: &DefinitionId) -> Option<DefinitionInfo> {
        match self.definitions.borrow_mut().entry(*id) {
            Entry::Occupied(entry) => Some(entry.get().clone()),
            Entry::Vacant(_) => None,
        }
    }

    pub fn diagnostics(&self) -> RefMut<'_, DiagnosticBag> {
        self.diagnostics.borrow_mut()
    }

    pub fn push_declartion(&self, id: DefinitionId, info: &Local) {
        let info = DefinitionInfo {
            pattern: info.pattern.0.clone(),
            kind: DefinitionKind::Declaration,
            span: info.pattern.1.clone(),
            expr: Some(*info.init.ptr.clone()),
            child: None,
        };

        self.definitions.borrow_mut().insert(id, info);
    }

    pub fn push_assignment(&self, id: DefinitionId, info: &Local) {
        let info = DefinitionInfo {
            pattern: info.pattern.0.clone(),
            kind: DefinitionKind::Assignment,
            span: info.pattern.1.clone(),
            expr: Some(*info.init.ptr.clone()),
            child: None,
        };

        self.definitions.borrow_mut().insert(id, info);
    }

    pub fn push_reference(&self, id: DefinitionId, info: &Ident) {
        let info = DefinitionInfo {
            pattern: info.0.clone(),
            kind: DefinitionKind::Reference,
            span: info.1.clone(),
            expr: None,
            child: None,
        };

        self.definitions.borrow_mut().insert(id, info);
    }

    pub fn push_child(&self, parent: &DefinitionId, child: &DefinitionId) {
        let mut id = *parent;
        while let Some(ref child) = self.definitions.borrow().get(&id).unwrap().child {
            id = *child;
            continue;
        }

        if let Some(def) = self.definitions.borrow_mut().get_mut(&id) {
            def.child = Some(*child);
        } // Handle None
    }
}
