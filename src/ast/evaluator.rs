use super::{visitor::{Visitor, Walkable}, BinOp, Expr};

pub struct AstEvaluator {
    pub last_value: Option<isize>,
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

    fn visit_literal(&mut self, lit: &super::Literal) {
        if let super::Literal::Integer(i) = lit {
            self.last_value = Some(*i);
        }
    }
}
