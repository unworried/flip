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

    pub fn get_variable_ref(&self, name: &str) -> Option<&DefinitionId> {
        self.variables.get(name)
    }

    pub fn define_variable(&mut self, name: String, id: DefinitionId) {
        self.variables.insert(name, id);
    }
}
