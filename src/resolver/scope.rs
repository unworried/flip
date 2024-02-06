use std::collections::HashMap;

use crate::cache::DefinitionId;

/*
 * Scope
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

#[derive(Debug)]
pub struct Scope {
    pub variables: HashMap<String, DefinitionId>,
    pub count: usize,
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
            count: 0,
        }
    }

    pub fn check_variable(&self, name: &str) -> bool {
        self.variables.get(name).is_some()
    }

    pub fn declare_variable(&mut self, name: String) -> DefinitionId {
        let id = self.count;
        self.count += 1;
        self.variables.insert(name, id);
        id
    }
}
