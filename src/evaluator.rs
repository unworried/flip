use crate::parser::ast::{BinOp, Binary, Literal, LiteralKind, UnOp, Unary, Variable};
use crate::parser::visitor::{Visitor, Walkable};

// Currently any operation that yields a float is floored.
#[derive(Default)]
pub struct Evaluator {
    pub last_value: Option<f64>,
}

impl Visitor for Evaluator {
    fn visit_variable(&mut self, _pat: &Variable) {
        self.last_value = None;
    }

    fn visit_binary(&mut self, bin: &Binary) {
        bin.left.walk(self);
        let left = match self.last_value {
            Some(value) => value,
            None => return,
        };

        bin.right.walk(self);
        let right = match self.last_value {
            Some(value) => value,
            None => return,
        };

        self.last_value = Some(match bin.op {
            BinOp::Add => left + right,
            BinOp::Sub => left - right,
            BinOp::Mul => left * right,
            BinOp::Div => left / right,
            _ => todo!("{:?}", bin.op),
        });
    }

    fn visit_unary(&mut self, un: &Unary) {
        un.operand.walk(self);
        let value = match self.last_value {
            Some(value) => value,
            None => return,
        };

        self.last_value = Some(match un.op {
            UnOp::Neg => -value,
        });
    }

    fn visit_literal(&mut self, lit: &Literal) {
        if let LiteralKind::Int(i) = lit.kind {
            self.last_value = Some(i as f64);
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
    use crate::diagnostics::DiagnosticBag;
    use crate::parser::Parser;

    fn assert_eval(input: &str, expected: f64) {
        let mut lexer = crate::lexer::Lexer::new(input.to_string());
        let diagnostics = DiagnosticBag::new();
        let mut parser = Parser::new(&mut lexer, diagnostics);
        let program = parser.parse();
        let mut evaluator = Evaluator::default();
        evaluator.visit_ast(&program);
        println!("{}", program);
        assert_eq!(evaluator.last_value.unwrap(), expected);
    }

    #[test]
    fn literal_int() {
        assert_eval("let x = 123;", 123.0);
    }

    #[test]
    fn unary() {
        assert_eval("let x = -123;", -123.0);
    }

    #[test]
    fn binary() {
        assert_eq!(1 + 2, 3);
        assert_eval("let x = 1 + 2;", 3.0);
        assert_eq!(1 - 2, -1);
        assert_eval("let x = 1 - 2;", -1.0);
        assert_eq!(1 * 2, 2);
        assert_eval("let x = 1 * 2;", 2.0);
        assert_eq!(2 / 1, 2);
        assert_eval("let x = 1 / 2;", 0.0);
    }

    #[test]
    fn complex() {
        assert_eq!(1 + 2 * 3, 7);
        assert_eval("let x = 1 + 2 * 3;", 7.0);
        assert_eq!(1 * 2 + 3, 5);
        assert_eval("let x = 1 * 2 + 3;", 5.0);
        assert_eq!(1 + 0 / 3, 1);
        assert_eval("let x = 1 + 0 / 3;", 1.0);
        assert_eq!(4 / 2 + 3, 5);
        assert_eval("let x = 2 / 1 + 3;", 5.0);
    }

    #[test]
    fn complex_unary() {
        assert_eq!(-1 + 2 * 3, 5);
        assert_eval("let x = -1 + 2 * 3;", 5.0);
        assert_eq!(-1 * 2 + 3, 1);
        assert_eval("let x = -1 * 2 + 3;", 1.0);
    }

    #[test]
    fn complex_paren() {
        assert_eq!((1 + 2) * 3, 9);
        assert_eval("let x = (1 + 2) * 3;", 9.0);
        assert_eq!(1 * (2 + 3), 5);
        assert_eval("let x = 1 * (2 + 3);", 5.0);
        assert_eq!(1 + (0 / 3), 1);
        assert_eval("let x = 1 + (0 / 3);", 1.0);
    }

    #[test]
    fn complex_paren_unary() {
        assert_eq!(-1 + (2 * 3), 5);
        assert_eval("let x = -1 + (2 * 3);", 5.0);
        assert_eq!(-1 * (2 + 3), -5);
        assert_eval("let x = -1 * (2 + 3);", -5.0);
    }

    #[test]
    fn complex_unary_parent() {
        assert_eq!(-(1 + 2) * 3, -9);
        assert_eval("let x = -(1 + 2) * 3;", -9.0);
        assert_eq!(-1 * -(-2 + 3), 1);
        assert_eval("let x = -1 * -(-2 + 3);", 1.0);
    }

    #[test]
    #[should_panic]
    fn divide_by_zero() {
        let input = "let x = 7 / (3 - 3);";
        let mut lexer = crate::lexer::Lexer::new(input.to_string());
        let diagnostics = DiagnosticBag::new();
        let mut parser = Parser::new(&mut lexer, diagnostics);
        let program = parser.parse();
        let mut evaluator = Evaluator::default();
        evaluator.visit_ast(&program);
        println!("{}", program);
    }

    #[test]
    fn fuzzy() {
        assert_eq!(----------------------------------------------45, 45);
        assert_eval(
            "let x = ----------------------------------------------45;",
            45.0,
        );
        assert_eq!(1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10, 55);
        assert_eval("let x = 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10;", 55.0);
        assert_eq!(1 + 2 * 3 + 4 * 5 + 6 * 7 + 8 * 9 + 10, 151);
        assert_eval("let x = 1 + 2 * 3 + 4 * 5 + 6 * 7 + 8 * 9 + 10;", 151.0);
        assert_eq!(1 * 2 + 3 * 4 + 5 * 6 + 7 * 8 + 9 * 10, 190);
        assert_eval("let x = 1 * 2 + 3 * 4 + 5 * 6 + 7 * 8 + 9 * 10;", 190.0);
        assert_eq!(1 + 2 * 3 * 4 + 5 * 6 * 7 + 8 * 9 * 10, 955);
        assert_eval("let x = 1 + 2 * 3 * 4 + 5 * 6 * 7 + 8 * 9 * 10;", 955.0);
        assert_eval(
            "let x = (((((((((((((((((((((((((((((1)))))))))))))))))))))))))))));",
            1.0,
        );
    }
}
