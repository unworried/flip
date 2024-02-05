use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use crate::{diagnostics::DiagnosticsCell, parser::ast::statement::Local, span::Span};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DefinitionId(pub usize);

#[derive(Debug, Clone)]
pub struct DefinitionInfo {
    pub pattern: String,
    pub kind: DefinitionKind,
    pub span: Span,
    pub parent: Option<DefinitionId>,
}

#[derive(Debug, Clone)]
pub enum DefinitionKind {
    Declaration,
    Definition,
}

// TODO: Add diagnostics only in cache (Centralize) then cache can be made on large Cell not
// individuals
pub struct Cache {
    pub definitions: RefCell<HashMap<DefinitionId, Rc<DefinitionInfo>>>,
    pub diagnostics: DiagnosticsCell,
}

impl Cache {
    pub fn new(diagnostics: DiagnosticsCell) -> Self {
        Self {
            definitions: RefCell::new(HashMap::new()),
            diagnostics,
        }
    }
}

impl Cache {
    pub fn get(&self, id: DefinitionId) -> Option<Rc<DefinitionInfo>> {
        match self.definitions.borrow_mut().entry(id) {
            Entry::Occupied(entry) => Some(entry.get().clone()),
            Entry::Vacant(_) => None,
        }
    }

    pub fn push_declartion(&self, id: DefinitionId, info: &Local) {
        let info = Rc::new(DefinitionInfo {
            pattern: info.pattern.0.clone(),
            kind: DefinitionKind::Declaration,
            span: info.pattern.1.clone(),
            parent: None,
        });

        self.definitions.borrow_mut().insert(id, info);
    }

    pub fn push_definition(&self, id: DefinitionId, info: &Local) {
        let info = Rc::new(DefinitionInfo {
            pattern: info.pattern.0.clone(),
            kind: DefinitionKind::Definition,
            span: info.pattern.1.clone(),
            parent: None,
        });

        self.definitions.borrow_mut().insert(id, info);
    }
}
