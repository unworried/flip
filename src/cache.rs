use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
};

use crate::{
    diagnostics::DiagnosticsCell,
    parser::ast::{statement::Local, Expr, Ident},
    span::Span,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DefinitionId(pub usize);

#[derive(Debug, Clone)]
pub struct DefinitionInfo {
    pub pattern: String,
    pub kind: DefinitionKind,
    pub span: Span,
    pub expr: Option<Expr>,
    pub parent: Option<DefinitionId>,
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
    pub diagnostics: DiagnosticsCell,
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
    pub fn get(&self, id: DefinitionId) -> Option<DefinitionInfo> {
        match self.definitions.borrow_mut().entry(id) {
            Entry::Occupied(entry) => Some(entry.get().clone()),
            Entry::Vacant(_) => None,
        }
    }

    pub fn push_declartion(&self, id: DefinitionId, info: &Local) {
        let info = DefinitionInfo {
            pattern: info.pattern.0.clone(),
            kind: DefinitionKind::Declaration,
            span: info.pattern.1.clone(),
            expr: Some(*info.init.ptr.clone()),
            parent: None,
        };

        self.definitions.borrow_mut().insert(id, info);
    }

    pub fn push_assignment(&self, id: DefinitionId, info: &Local) {
        let info = DefinitionInfo {
            pattern: info.pattern.0.clone(),
            kind: DefinitionKind::Assignment,
            span: info.pattern.1.clone(),
            expr: Some(*info.init.ptr.clone()),
            parent: None,
        };

        self.definitions.borrow_mut().insert(id, info);
    }

    pub fn push_reference(&self, id: DefinitionId, info: &Ident) {
        let info = DefinitionInfo {
            pattern: info.0.clone(),
            kind: DefinitionKind::Reference,
            span: info.1.clone(),
            expr: None,
            parent: None,
        };

        self.definitions.borrow_mut().insert(id, info);
    }

    pub fn push_parent(&self, parent: &DefinitionId, child: &DefinitionId) {
        self.definitions.borrow_mut().get_mut(child).unwrap().parent = Some(parent.clone());
    }
}
