use std::collections::HashMap;

use crate::ast::visitor::{Visitor, Walkable};
use crate::ast::{
    Assignment, Ast, Binary, Call, Definition, If, Literal, LiteralKind, Unary, Variable, While,
};
use crate::diagnostics::DiagnosticBag;
use crate::lexer::{Lexer, Token};
use crate::parser::combinators::parse_sequence;
use crate::parser::Parser;
use crate::source::Source;

pub fn assert_ast(input: &str, expected: Vec<ASTNode>) {
    let validator = AstValidator::new(input, expected);
    validator.validate();
}

// HashMap<function_name, expected_ast>
pub fn assert_program(input: &str, expected: HashMap<String, Vec<ASTNode>>) {
    let mut lexer = Lexer::new(input.to_string());
    let diagnostics = DiagnosticBag::new();
    let mut parser = Parser::new(&mut lexer, diagnostics.clone());
    let program = parser.parse();

    for func in program.functions {
        let mut validator = AstValidator {
            expected: expected.get(&func.pattern.name).unwrap().to_vec(),
            actual: Vec::new(),
        };
        // TODO: May revist this, pushes paramters to top of ast to compare with expected
        for param in &func.parameters {
            validator
                .actual
                .push(ASTNode::Variable(param.name.to_owned()));
        }

        validator.flatten_ast(&func.body);
        validator.validate();
    }

    // TODO: Maybe refactor this
    // FIXME: Will break if diagnostic messages change
    let diagnostic_msgs: Vec<String> = diagnostics
        .borrow()
        .errors
        .iter()
        .map(|d| d.message.clone())
        .collect();
    assert!(
        diagnostics.borrow().is_empty(),
        "diagnostics returned: {:?}",
        diagnostic_msgs
    );
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    If,
    While,
    Let,
    Integer(u64),
    String(String),
    Binary,
    Unary,
    Variable(String),
    Call(String),
}

pub struct AstValidator {
    expected: Vec<ASTNode>,
    actual: Vec<ASTNode>,
}

impl AstValidator {
    // FIXME: Diagnostics not being checked
    pub fn new(input: &str, expected: Vec<ASTNode>) -> Self {
        let mut lexer = Lexer::new(input.to_string());
        let diagnostics = DiagnosticBag::new();
        let mut parser = Parser::new(&mut lexer, diagnostics);
        let ast = parse_sequence(&mut parser, Token::Eof);
        let mut validator = AstValidator {
            expected,
            actual: Vec::new(),
        };
        validator.flatten_ast(&ast);

        validator
    }

    fn flatten_ast(&mut self, ast: &Ast) {
        //self.actual.clear(); // FIXME: Do i need this?
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
    fn visit_call(&mut self, call: &Call) {
        self.actual
            .push(ASTNode::Call(call.pattern.name.to_owned()));
        call.arguments.iter().for_each(|arg| arg.walk(self));
    }

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
        self.actual.push(ASTNode::Variable(var.name.to_owned()));
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
