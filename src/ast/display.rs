use crate::{lexer::Lexer, parser::Parser};

use super::{visitor::Visitor, Stmt, StmtKind};

pub struct AstDisplay {
    ident: usize,
    result: Vec<Stmt>,
}

impl AstDisplay {
    pub fn new() -> Self {
        Self {
            ident: 0,
            result: Vec::new(),
        }
    }

    pub fn test() {
        let input = "PRINT \"TEST\"\nIF 1 == 1 THEN\nLET foo = 45\nENDIF\n";
        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);

        let ast = parser.parse();

        let mut display = Self::new();
        display.visit_ast(&ast);
    }
}

impl Visitor for AstDisplay {
    fn visit_stmt(&mut self, node: &Stmt) {
        let StmtKind::If(.., ref body) = node.kind else {
            return;
        };

        println!("{:#?}", body);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        AstDisplay::test();
        panic!();
    }
}

