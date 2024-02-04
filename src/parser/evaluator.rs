use super::{
    ast::{BinOp, Expr, Literal, UnOp},
    visitor::{Visitor, Walkable},
};

// Currently any operation that yields a float is floored.
#[derive(Default)]
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

    fn visit_unary(&mut self, op: &UnOp, expr: &Expr) {
        expr.walk(self);
        let value = self.last_value.unwrap();
        self.last_value = Some(match op {
            UnOp::Neg => -value,
        });
    }

    fn visit_literal(&mut self, lit: &Literal) {
        if let Literal::Integer(i) = lit {
            self.last_value = Some(*i);
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::identity_op,
        clippy::erasing_op,
        clippy::neg_multiply,
        clippy::double_neg
    )]

    use super::*;
    use crate::{diagnostics::DiagnosticBag, parser::Parser};

    fn assert_eval(input: &str, expected: isize) {
        let mut lexer = crate::lexer::Lexer::new(input.to_string());
        let diagnostics = DiagnosticBag::new();
        let mut parser = Parser::new(&mut lexer, diagnostics);
        let program = parser.parse();
        let mut evaluator = AstEvaluator::default();
        evaluator.visit_ast(&program);
        println!("{}", program);
        assert_eq!(evaluator.last_value.unwrap(), expected);
    }

    #[test]
    fn literal_int() {
        assert_eval("let x = 123;", 123);
    }

    #[test]
    fn unary() {
        assert_eval("let x = -123;", -123);
    }

    #[test]
    fn binary() {
        assert_eq!(1 + 2, 3);
        assert_eval("let x = 1 + 2;", 3);
        assert_eq!(1 - 2, -1);
        assert_eval("let x = 1 - 2;", -1);
        assert_eq!(1 * 2, 2);
        assert_eval("let x = 1 * 2;", 2);
        assert_eq!(2 / 1, 2);
        assert_eval("let x = 1 / 2;", 0);
    }

    #[test]
    fn complex() {
        assert_eq!(1 + 2 * 3, 7);
        assert_eval("let x = 1 + 2 * 3;", 7);
        assert_eq!(1 * 2 + 3, 5);
        assert_eval("let x = 1 * 2 + 3;", 5);
        assert_eq!(1 + 0 / 3, 1);
        assert_eval("let x = 1 + 0 / 3;", 1);
        assert_eq!(4 / 2 + 3, 5);
        assert_eval("let x = 2 / 1 + 3;", 5);
    }

    #[test]
    fn complex_unary() {
        assert_eq!(-1 + 2 * 3, 5);
        assert_eval("let x = -1 + 2 * 3;", 5);
        assert_eq!(-1 * 2 + 3, 1);
        assert_eval("let x = -1 * 2 + 3;", 1);
    }

    #[test]
    fn complex_paren() {
        assert_eq!((1 + 2) * 3, 9);
        assert_eval("let x = (1 + 2) * 3;", 9);
        assert_eq!(1 * (2 + 3), 5);
        assert_eval("let x = 1 * (2 + 3);", 5);
        assert_eq!(1 + (0 / 3), 1);
        assert_eval("let x = 1 + (0 / 3);", 1);
    }

    #[test]
    fn complex_paren_unary() {
        assert_eq!(-1 + (2 * 3), 5);
        assert_eval("let x = -1 + (2 * 3);", 5);
        assert_eq!(-1 * (2 + 3), -5);
        assert_eval("let x = -1 * (2 + 3);", -5);
    }

    #[test]
    fn complex_unary_parent() {
        assert_eq!(-(1 + 2) * 3, -9);
        assert_eval("let x = -(1 + 2) * 3;", -9);
        assert_eq!(-1 * -(-2 + 3), 1);
        assert_eval("let x = -1 * -(-2 + 3);", 1);
    }

    #[test]
    #[should_panic]
    fn divide_by_zero() {
        let input = "let x = 7 / (3 - 3);";
        let mut lexer = crate::lexer::Lexer::new(input.to_string());
        let diagnostics = DiagnosticBag::new();
        let mut parser = Parser::new(&mut lexer, diagnostics);
        let program = parser.parse();
        let mut evaluator = AstEvaluator::default();
        evaluator.visit_ast(&program);
        println!("{}", program);
    }

    #[test]
    fn fuzzy() {
        assert_eq!(----------------------------------------------45, 45);
        assert_eval(
            "let x = ----------------------------------------------45;",
            45,
        );
        assert_eq!(1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10, 55);
        assert_eval("let x = 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10;", 55);
        assert_eq!(1 + 2 * 3 + 4 * 5 + 6 * 7 + 8 * 9 + 10, 151);
        assert_eval("let x = 1 + 2 * 3 + 4 * 5 + 6 * 7 + 8 * 9 + 10;", 151);
        assert_eq!(1 * 2 + 3 * 4 + 5 * 6 + 7 * 8 + 9 * 10, 190);
        assert_eval("let x = 1 * 2 + 3 * 4 + 5 * 6 + 7 * 8 + 9 * 10;", 190);
        assert_eq!(1 + 2 * 3 * 4 + 5 * 6 * 7 + 8 * 9 * 10, 955);
        assert_eval("let x = 1 + 2 * 3 * 4 + 5 * 6 * 7 + 8 * 9 * 10;", 955);
        assert_eval(
            "let x = (((((((((((((((((((((((((((((1)))))))))))))))))))))))))))));",
            1,
        );
    }
}
