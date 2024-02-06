//! cache.rs - Module storing the global cache for the compiler frontend. The cache is used to
//! store the symbols table, links and the diagnostics stack.
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

use crate::diagnostics::{DiagnosticBag, DiagnosticsCell};
use crate::parser::ast::statement::Definition;

/*
 * MemCache
 * ------
 * Need to implement this as a hash table or btree to allow for fast lookups.
 * This will be used to store the variables in the current scope.
 *
 * Index | Symbol | Type | HashMap<EndSpan, Value>
 * e.g.
 * 0     | x      | i64  | { 5: 10, 17: 11, 20: 12 }
 *
 * HashMap<Symbol, Index> Kept in scope. This will be used to look up the index of the variable
 * and check the current value of the variable dependent on the current span.
 *
 */

pub type DefinitionId = usize;

// TODO: Add diagnostics only in cache (Centralize) then cache can be made on large Cell not
// individuals
pub struct Cache {
    pub definitions: RefCell<HashMap<DefinitionId, Definition>>,
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
    pub fn diagnostics(&self) -> RefMut<'_, DiagnosticBag> {
        self.diagnostics.borrow_mut()
    }

    pub fn lookup(&self, id: &DefinitionId) -> Option<Definition> {
        self.definitions.borrow().get(id).cloned()
    }

    pub fn push_definition(&self, definition: &Definition) -> DefinitionId {
        let id  = self.definitions.borrow().len();
        self.definitions.borrow_mut().insert(id, definition.clone());
        id
    }

    /*pub fn push_reference(&self, id: DefinitionId, info: &Ident) {
        let info = DefinitionInfo {
            pattern: info.0.clone(),
            kind: DefinitionKind::Reference,
            span: info.1.clone(),
            expr: None,
            child: None,
        };

        self.definitions.borrow_mut().insert(id, info);
    }*/

    /*pub fn push_child(&self, parent: &DefinitionId, child: &DefinitionId) {
        let mut id = *parent;

        while let Some(child) = self.definitions.borrow().get(&id).and_then(|def| def.child) {
            id = child;
        }

        if let Some(def) = self.definitions.borrow_mut().get_mut(&id) {
            def.child = Some(*child);
        } // Handle None
    }*/
}
