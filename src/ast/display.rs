use std::fmt::Display;

use super::{
    visitor::{Visitor, Walkable}, Ast, ExprKind, Literal, Stmt, StmtKind
};

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
}

impl Visitor for AstDisplay {
    fn visit_stmt(&mut self, stmt: &Stmt) {
        self.add_padding();
        self.result.push_str("Statement: ");
        stmt.walk(self);
        self.ident -= 1;
    }

    fn visit_stmt_kind(&mut self, stmt: &StmtKind) {
        self.ident += 1;
        match stmt {
            StmtKind::Let(..) => self.result.push_str("let"),
            StmtKind::If(..) => self.result.push_str("if"),
            StmtKind::While(..) => self.result.push_str("while"),
        }

        stmt.walk(self);
        self.add_newline();
    }

    fn visit_expr(&mut self, expr: &super::Expr) {
        self.add_newline();
        self.add_padding();
        self.result.push_str("Expression: ");
        expr.walk(self);
    }

    fn visit_expr_kind(&mut self, expr: &super::ExprKind) {
        self.ident += 1;
        match expr {
            ExprKind::Unary(op, ..) => {
                self.add_newline();
                self.add_padding();
                self.result.push_str(&format!("Unary: {:?}", op));
            },
            ExprKind::Binary(op, ..) => {
                self.add_newline();
                self.add_padding();
                self.result.push_str(&format!("Binary: {:?}", op));
            },
            ExprKind::Literal(lit) => lit.walk(self),
            ExprKind::Ident(s) => self.result.push_str(&s.to_string()),
        }

        expr.walk(self);
        self.ident -= 1;
    }

    fn visit_binary(&mut self, _op: &super::BinOp, lhs: &super::Expr, rhs: &super::Expr) {
        self.ident += 1;
        self.add_newline();
        self.add_padding();
        self.result.push_str("Left: ");
        lhs.walk(self);
        self.add_newline();
        self.add_padding();
        self.result.push_str("Right: ");
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
