use alloc::string::String;

use crate::error::{CompilerError, Result};

#[derive(Debug)]
pub struct Source {
    text: String,
}

impl Source {
    pub fn new(text: String) -> Source {
        Source { text }
    }

    pub fn line_index(&self, index: usize) -> usize {
        self.text[..index].chars().filter(|&c| c == '\n').count()
    }

    pub fn line(&self, index: usize) -> Result<&str> {
        self.text
            .lines()
            .nth(index)
            .ok_or(CompilerError::ReadSource)
    }

    pub fn line_start(&self, index: usize) -> usize {
        self.text
            .lines()
            .take(index)
            .map(|line| line.len() + 1)
            .sum()
    }
}
