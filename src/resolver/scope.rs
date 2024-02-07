use std::collections::HashMap;

use crate::resolver::DefinitionId;

#[derive(Debug, Default)]
pub struct Scope {
    pub variables: HashMap<String, DefinitionId>,
}

impl Scope {
    pub fn lookup_symbol(&self, name: &str) -> Option<&DefinitionId> {
        self.variables.get(name)
    }

    pub fn define_symbol(&mut self, name: &str, id: DefinitionId) {
        self.variables.insert(name.to_owned(), id);
    }
}
