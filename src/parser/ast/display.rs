use alloc::string::{String, ToString};
use core::fmt::{Display, Formatter, Result};

use crate::{
    escape_codes::Color,
    parser::visitor::{Visitor, Walkable},
};

use super::{Ast, ExprKind, Literal, Stmt, StmtKind};

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
    fn visit_stmt(&mut self, stmt: &Stmt) {
        self.add_statement_header();
        stmt.walk(self);
        self.ident -= 1;
        self.add_newline();
        self.add_newline();
        self.add_padding();
    }

    fn visit_stmt_kind(&mut self, stmt: &StmtKind) {
        self.ident += 1;
        match stmt {
            StmtKind::Let(local) => {
                self.result.push_str("Declare ");
                self.result.push_str(&local.ptr.pattern.0);
                self.add_newline();
                self.add_padding();
                self.add_expression_header("Expression");
            }

            StmtKind::If(..) => self.result.push_str("If"),
            StmtKind::While(..) => self.result.push_str("While"),
            StmtKind::Assignment(local) => {
                self.result.push_str("Assign ");
                self.result.push_str(&local.ptr.pattern.0);
                self.add_newline();
                self.add_padding();
                self.add_expression_header("Expression");
            }
            StmtKind::Error => self.result.push_str("Error"),
        }

        stmt.walk(self);
    }

    fn visit_expr(&mut self, expr: &super::Expr) {
        self.add_newline();
        self.add_padding();
        self.add_expression_header("Expression");
        expr.walk(self);
    }

    fn visit_expr_kind(&mut self, expr: &super::ExprKind) {
        self.ident += 1;
        match expr {
            ExprKind::Unary(op, ..) => {
                self.add_newline();
                self.add_padding();
                self.add_expression_header("Unary");
                self.result.push_str(&format!("{:?}", op));
            }

            ExprKind::Binary(op, ..) => {
                self.add_newline();
                self.add_padding();
                self.add_expression_header("Binary");
                self.result.push_str(&format!("{:?}", op));
            }

            ExprKind::Literal(lit) => lit.walk(self),
            ExprKind::Variable(s) => self.result.push_str(&s.0.to_string()),
            ExprKind::Error => self.result.push_str("Error"),
        }

        expr.walk(self);
        self.ident -= 1;
    }

    fn visit_binary(&mut self, _op: &super::BinOp, lhs: &super::Expr, rhs: &super::Expr) {
        self.ident += 1;
        self.add_newline();
        self.add_padding();
        self.add_expression_header("Left");
        lhs.walk(self);
        self.add_newline();
        self.add_padding();
        self.add_expression_header("Right");
        rhs.walk(self);
        self.ident -= 1;
    }

    fn visit_unary(&mut self, _op: &super::UnOp, expr: &super::Expr) {
        self.ident += 1;
        self.add_newline();
        self.add_padding();
        self.result.push_str("Value: ");
        expr.walk(self);
        self.ident -= 1;
    }

    fn visit_literal(&mut self, lit: &Literal) {
        match lit {
            Literal::Integer(i) => self.result.push_str(&i.to_string()),
            Literal::String(s) => self.result.push_str(&s.to_string()),
        }
    }
}
