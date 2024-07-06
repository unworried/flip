use crate::ast::visitor::Walkable;
use crate::ast::Ast;
use alloc::string::String;
use core::fmt::{Display, Formatter, Result};

use crate::escape_codes::Color;

use super::visitor::Visitor;
use super::{Assignment, Binary, Definition, If, Literal, LiteralKind, Unary, Variable, While};

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut display = AstDisplay::new();
        write!(f, "{}", display.build(self))
    }
}

pub struct AstDisplay {
    indent: usize,
    result: String,
}

impl AstDisplay {
    pub fn new() -> Self {
        Self {
            indent: 0,
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
        for _ in 0..self.indent {
            self.result.push_str("  ");
        }
    }

    fn add_statement_header(&mut self, text: &str) {
        self.add_newline();
        self.add_padding();
        self.result
            .push_str(&format!("{}{}:{} ", Color::Magenta, text, Color::Reset));
    }

    fn add_expression_header(&mut self, text: &str) {
        self.add_newline();
        self.add_padding();
        self.result
            .push_str(&format!("{}{}:{} ", Color::Cyan, text, Color::Reset));
    }

    fn add_block_end(&mut self) {
        self.indent -= 2;
        self.add_newline();
        self.add_padding();
        self.result
            .push_str(&format!("{}End{} ", Color::Magenta, Color::Reset));
        self.add_newline();
        self.add_padding();
    }
}

impl Visitor for AstDisplay {
    fn visit_definition(&mut self, def: &Definition) {
        self.add_statement_header("Declare");
        //self.result.push_str(&format!("{}({:?})", def.pattern.name, def.id));
        self.result.push_str(&def.pattern.name);

        self.indent += 1;
        self.add_expression_header("Expression");
        def.value.walk(self);

        self.indent -= 1;
    }

    fn visit_assignment(&mut self, def: &Assignment) {
        self.add_statement_header("Assign");
        //self.result.push_str(&format!("{}({:?})", def.pattern.name, def.id));
        self.result.push_str(&def.pattern.name);

        self.indent += 1;
        self.add_expression_header("Expression");
        def.value.walk(self);

        self.indent -= 1;
    }

    fn visit_binary(&mut self, bin: &Binary) {
        self.indent += 1;
        self.add_expression_header("Left");
        bin.left.walk(self);
        self.add_expression_header("Op");
        self.result.push_str(&format!("{:?}", bin.op));
        self.add_expression_header("Right");
        bin.right.walk(self);
        self.indent -= 1;
    }

    fn visit_unary(&mut self, un: &Unary) {
        self.indent += 1;
        self.add_expression_header("Op");
        self.result.push_str(&format!("{:?}", un.op));
        self.add_expression_header("Operand");
        un.operand.walk(self);
        self.indent -= 1;
    }

    fn visit_literal(&mut self, lit: &Literal) {
        match &lit.kind {
            LiteralKind::Int(i) => self.result.push_str(&i.to_string()),
            LiteralKind::String(s) => self.result.push_str(&format!("\"{}\"", s)),
        }
    }

    fn visit_variable(&mut self, var: &Variable) {
        //self.result.push_str(&format!("{}({:?})", var.pattern, var.definition));
        self.result.push_str(&var.name);
    }

    fn visit_while(&mut self, while_expr: &While) {
        self.add_newline();
        self.add_statement_header("While");
        self.indent += 1;
        self.add_statement_header("Condition");
        self.indent += 1;
        self.add_expression_header("Expression");
        while_expr.condition.walk(self);
        self.indent -= 1;

        self.add_newline();
        self.add_statement_header("Then");
        self.indent += 1;
        while_expr.then.walk(self);
        self.add_block_end();
    }

    fn visit_if(&mut self, if_expr: &If) {
        self.add_newline();
        self.add_statement_header("If");
        self.indent += 1;
        self.add_statement_header("Condition");
        self.indent += 1;
        self.add_expression_header("Expression");
        if_expr.condition.walk(self);
        self.indent -= 1;

        self.add_newline();
        self.add_statement_header("Then");
        self.indent += 1;
        if_expr.then.walk(self);
        self.add_block_end();
    }
}
