use alloc::string::String;
use core::fmt::{Display, Formatter, Result};

use crate::escape_codes::Color;

use super::ast::{Ast, Binary, Definition, If, Literal, LiteralKind, Unary, Variable, While};
use super::visitor::{Visitor, Walkable};

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut display = AstDisplay::new();
        write!(f, "{}", display.build(self))
    }
}

pub struct AstDisplay {
    ident: usize,
    result: String,
}

impl AstDisplay {
    pub fn new() -> Self {
        Self {
            ident: 0,
            result: String::new(),
        }
    }

    pub fn build(&mut self, ast: &Ast) -> &String {
        self.visit_ast(ast);
        &self.result
    }

    fn add_newline(&mut self) {
        self.result.push('\n');
    }

    fn add_padding(&mut self) {
        for _ in 0..self.ident {
            self.result.push_str("  ");
        }
    }

    fn add_statement_header(&mut self) {
        self.result
            .push_str(&format!("{}Statement:{} ", Color::Magenta, Color::Reset));
    }

    fn add_expression_header(&mut self, text: &str) {
        self.result
            .push_str(&format!("{}{}:{} ", Color::Cyan, text, Color::Reset));
    }
}

impl Visitor for AstDisplay {
    fn visit_definition(&mut self, def: &Definition) {
        self.add_newline();
        self.add_padding();
        self.ident += 1;
        self.add_statement_header();
        self.result.push_str("Declare ");
        self.result.push_str(&def.pattern.0);

        self.add_newline();
        self.add_padding();
        self.add_expression_header("Expression");
        def.value.walk(self);

        self.ident -= 1;
        self.add_newline();
        self.add_padding();
    }

    fn visit_assignment(&mut self, def: &Definition) {
        self.add_newline();
        self.add_padding();
        self.add_statement_header();
        self.result.push_str("Assign ");
        self.result.push_str(&def.pattern.0);

        self.ident += 1;
        self.add_newline();
        self.add_padding();
        self.add_expression_header("Expression");
        def.value.walk(self);

        self.ident -= 1;
        self.add_newline();
        self.add_padding();
    }

    fn visit_binary(&mut self, bin: &Binary) {
        self.ident += 1;
        self.add_newline();
        self.add_padding();
        self.add_expression_header("Left");
        bin.left.walk(self);
        self.add_newline();
        self.add_padding();
        self.add_expression_header("Op");
        self.result.push_str(&format!("{:?}", bin.op));
        self.add_newline();
        self.add_padding();
        self.add_expression_header("Right");
        bin.right.walk(self);
        self.ident -= 1;
        self.add_newline();
        self.add_padding();
    }

    fn visit_unary(&mut self, un: &Unary) {
        self.add_newline();
        self.add_padding();
        self.add_expression_header("Expression");
        self.ident += 1;
        self.add_newline();
        self.add_padding();
        self.result.push_str("Value: ");
        un.oprand.walk(self);
        self.ident -= 1;
        self.add_newline();
        self.add_padding();
    }

    fn visit_literal(&mut self, lit: &Literal) {
        match &lit.kind {
            LiteralKind::Int(i) => self.result.push_str(&i.to_string()),
            LiteralKind::String(s) => self.result.push_str(&s.to_string()),
        }
    }

    fn visit_variable(&mut self, var: &Variable) {
        self.result.push_str(&var.pattern.0);
    }

    fn visit_while(&mut self, while_expr: &While) {
        self.add_newline();
        self.add_padding();
        self.add_statement_header();
        self.result.push_str("While");
        self.ident += 1;
        self.add_newline();
        self.add_padding();
        self.add_expression_header("Expression");

        while_expr.condition.walk(self);

        self.add_newline();
        self.add_padding();
        self.result
            .push_str(&format!("{}Then:{} ", Color::Magenta, Color::Reset));
        self.ident += 1;
        while_expr.then.walk(self);
        self.ident -= 2;
    }

    fn visit_if(&mut self, if_expr: &If) {
        self.add_newline();
        self.add_padding();
        self.add_statement_header();
        self.result.push_str("If");
        self.ident += 1;
        self.add_newline();
        self.add_padding();
        self.add_expression_header("Expression");

        if_expr.condition.walk(self);

        self.add_newline();
        self.add_padding();
        self.result
            .push_str(&format!("{}Then:{} ", Color::Magenta, Color::Reset));
        self.ident += 1;
        if_expr.then.walk(self);
        self.ident -= 2;
    }
}
