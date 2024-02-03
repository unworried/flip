use super::{visitor::{Visitor, Walkable}, BinOp, Expr, UnOp};

pub struct AstEvaluator {
    pub last_value: Option<isize>,
}

impl AstEvaluator {
    pub fn new() -> Self {
        Self { last_value: None }
    }
}

impl Visitor for AstEvaluator {
    fn visit_binary(&mut self, op: &BinOp, lhs: &Expr, rhs: &Expr) {
        lhs.walk(self);
        let left = self.last_value.unwrap();
        rhs.walk(self);
        let right = self.last_value.unwrap();
        self.last_value = Some(match op {
            BinOp::Add => left + right,
            BinOp::Sub => left - right,
            BinOp::Mul => left * right,
            BinOp::Div => left / right,
            _ => todo!("{:?}", op),
        });
    }

    fn visit_unary(&mut self, op: &super::UnOp, expr: &Expr) {
        expr.walk(self);
        let value = self.last_value.unwrap();
        self.last_value = Some(match op {
            UnOp::Neg => -value,
        });
    }

    fn visit_literal(&mut self, lit: &super::Literal) {
        if let super::Literal::Integer(i) = lit {
            self.last_value = Some(*i);
        }
    }
}
