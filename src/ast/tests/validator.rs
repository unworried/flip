use crate::{
    ast::{
        visitor::{Visitor, Walkable},
        Ast, ExprKind, StmtKind,
    },
    lexer::Lexer,
    parser::Parser,
};

pub fn assert_ast(input: &str, expected: Vec<ASTNode>) {
    let validator = AstValidator::new(input, expected);
    validator.validate();
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    If,
    While,
    Let,
    Integer(isize),
    String(String),
    Binary,
    Unary,
    Ident(String),
}

pub struct AstValidator {
    expected: Vec<ASTNode>,
    actual: Vec<ASTNode>,
}

impl AstValidator {
    pub fn new(input: &str, expected: Vec<ASTNode>) -> Self {
        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);
        let ast = parser.parse();
        let mut validator = AstValidator {
            expected,
            actual: Vec::new(),
        };
        validator.flatten_ast(&ast);

        validator
    }

    fn flatten_ast(&mut self, ast: &Ast) {
        self.actual.clear();
        self.visit_ast(ast);
    }

    pub fn validate(&self) {
        assert_eq!(
            self.expected.len(),
            self.actual.len(),
            "expected: {:?} nodes, actual: {:?}",
            self.expected.len(),
            self.actual.len()
        );

        for (i, (expected, actual)) in self.expected.iter().zip(self.actual.iter()).enumerate() {
            assert_eq!(
                expected, actual,
                "expected: {:?}, actual: {:?} at index: {:?}",
                expected, actual, i
            );
        }
    }
}

impl Visitor for AstValidator {
    fn visit_stmt_kind(&mut self, node: &StmtKind) {
        match &node {
            StmtKind::If(cond, res) => {
                self.actual.push(ASTNode::If);
                cond.walk(self);
                for item in res {
                    item.walk(self);
                }
            }
            StmtKind::While(cond, res) => {
                self.actual.push(ASTNode::While);
                cond.walk(self);
                for item in res {
                    item.walk(self);
                }
            }
            StmtKind::Let(ident, expr) => {
                self.actual.push(ASTNode::Let);
                self.actual.push(ASTNode::Ident(ident.to_owned()));
                expr.walk(self);
            }
        }
    }

    fn visit_expr_kind(&mut self, node: &ExprKind) {
        match &node {
            ExprKind::Binary(.., lhs, rhs) => {
                self.actual.push(ASTNode::Binary);
                lhs.ptr.walk(self);
                rhs.ptr.walk(self);
            }
            ExprKind::Unary(.., int) => {
                self.actual.push(ASTNode::Unary);
                int.ptr.walk(self);
            }
            ExprKind::Literal(value) => value.walk(self),
            ExprKind::Ident(ident) => self.actual.push(ASTNode::Ident(ident.to_owned())),
        }
    }

    fn visit_integer_literal(&mut self, node: &isize) {
        self.actual.push(ASTNode::Integer(node.to_owned()));
    }

    fn visit_string_literal(&mut self, node: &str) {
        self.actual.push(ASTNode::String(node.to_owned()));
    }
}
