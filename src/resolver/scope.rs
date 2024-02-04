use crate::idx;
use crate::resolver::idxvec::Idx;

use super::idxvec::IdxVec;

idx!(VariableIdx);

#[derive(Debug)]
pub struct Scope {
    pub variables: IdxVec<VariableIdx, String>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: IdxVec::new(),
        }
    }

    pub fn declare_variable(&mut self, name: String) -> VariableIdx {
        self.variables.push(name)
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}



