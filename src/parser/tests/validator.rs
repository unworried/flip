use crate::{
    diagnostics::DiagnosticBag,
    lexer::Lexer,
    parser::{
        ast::{Ast, ExprKind, Literal, StmtKind},
        visitor::{Visitor, Walkable},
        Parser,
    },
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
    Variable(String),
}

pub struct AstValidator {
    expected: Vec<ASTNode>,
    actual: Vec<ASTNode>,
}

impl AstValidator {
    pub fn new(input: &str, expected: Vec<ASTNode>) -> Self {
        let mut lexer = Lexer::new(input.to_string());
        let diagnostics = DiagnosticBag::new();
        let mut parser = Parser::new(&mut lexer, diagnostics);
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
            StmtKind::Let(local) => {
                self.actual.push(ASTNode::Let);
                self.actual
                    .push(ASTNode::Variable(local.pattern.0.to_owned()));
                local.ptr.init.ptr.walk(self);
            }
            StmtKind::Assignment(local) => {
                self.actual
                    .push(ASTNode::Variable(local.pattern.0.to_owned()));
                local.ptr.init.ptr.walk(self);
            }
            StmtKind::Error => {}
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
            ExprKind::Literal(value) => self.visit_literal(value),
            ExprKind::Variable(ident) => self.actual.push(ASTNode::Variable(ident.0.to_owned())),
            ExprKind::Error => {}
        }
    }

    fn visit_literal(&mut self, lit: &Literal) {
        match &lit {
            Literal::Integer(int) => self.actual.push(ASTNode::Integer(int.to_owned())),
            Literal::String(string) => self.actual.push(ASTNode::String(string.to_owned())),
        }
    }
}
