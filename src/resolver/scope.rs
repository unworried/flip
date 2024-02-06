use std::collections::HashMap;

use crate::cache::DefinitionId;

#[derive(Debug)]
pub struct Scope {
    pub variables: HashMap<String, DefinitionId>,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn check_variable(&self, name: &str) -> bool {
        self.variables.get(name).is_some()
    }

    pub fn declare_variable(&mut self, name: String) -> DefinitionId {
        let id = self.variables.len();
        self.variables.insert(name, id);
        id
    }
}
