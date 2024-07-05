use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::nameresolver::DefinitionId;

#[derive(Debug, Default, Clone)]
pub struct Scope {
    pub parent: Option<Rc<RefCell<Scope>>>,
    pub variables: HashMap<String, DefinitionId>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Self {
        Self {
            parent,
            variables: HashMap::new(),
        }
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<DefinitionId> {
        self.variables.get(name).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.borrow().lookup_symbol(name))
        })
    }

    pub fn define_symbol(&mut self, name: &str, id: DefinitionId) -> bool {
        if !self.variables.contains_key(name) {
            self.variables.insert(name.to_owned(), id);
            true
        } else {
            false
        }
    }
}
