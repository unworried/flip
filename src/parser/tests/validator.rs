use crate::diagnostics::DiagnosticBag;
use crate::lexer::Lexer;
use crate::parser::ast::{
    Assignment, Ast, Binary, Definition, If, Literal, LiteralKind, Unary, Variable, While,
};
use crate::parser::visitor::{Visitor, Walkable};
use crate::parser::Parser;

pub fn assert_ast(input: &str, expected: Vec<ASTNode>) {
    let validator = AstValidator::new(input, expected);
    validator.validate();
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    If,
    While,
    Let,
    Integer(u64),
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
        println!("{:?}", self.actual);
        println!("{:?}", self.expected);
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
    fn visit_binary(&mut self, bin: &Binary) {
        self.actual.push(ASTNode::Binary);
        bin.left.walk(self);
        bin.right.walk(self);
    }

    fn visit_unary(&mut self, un: &Unary) {
        self.actual.push(ASTNode::Unary);
        un.operand.walk(self);
    }

    fn visit_definition(&mut self, def: &Definition) {
        self.actual.push(ASTNode::Let);
        self.actual
            .push(ASTNode::Variable(def.pattern.name.to_owned()));
        def.value.walk(self);
    }

    fn visit_assignment(&mut self, def: &Assignment) {
        self.actual
            .push(ASTNode::Variable(def.pattern.name.to_owned()));
        def.value.walk(self);
    }

    fn visit_variable(&mut self, var: &Variable) {
        self.actual.push(ASTNode::Variable(var.pattern.to_owned()));
    }

    fn visit_if(&mut self, if_expr: &If) {
        self.actual.push(ASTNode::If);
        if_expr.condition.walk(self);
        if_expr.then.walk(self);
    }

    fn visit_while(&mut self, while_expr: &While) {
        self.actual.push(ASTNode::While);
        while_expr.condition.walk(self);
        while_expr.then.walk(self);
    }

    fn visit_literal(&mut self, lit: &Literal) {
        match &lit.kind {
            LiteralKind::Int(int) => self.actual.push(ASTNode::Integer(int.to_owned())),
            LiteralKind::String(string) => self.actual.push(ASTNode::String(string.to_owned())),
        }
    }
}
