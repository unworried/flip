use crate::{
    lexer::Token,
    parser::{Parse, Parser, P},
};

use super::{Expr, ExprKind};

impl Expr {
    /// Grammar: (expression) (operator) (expression)
    /*
     * TODO: fix support for P<Expr> for lhs
     * lhs: atomic, rhs: subatomic/branching
     */
    pub fn parse_binary(parser: &mut Parser) -> ExprKind {
        let leftkind = match &parser.current_token {
            Token::Int(_) => Self::parse_literal(parser), // TODO: change
            Token::Ident(_) => Self::parse_ident(parser),
            _ => unimplemented!("Unexpected token {:?}", parser.current_token),
        };
        let left = P(Expr { kind: leftkind });

        parser.step();

        // CHECK: Will this retain the correct precedence?
        let operator = match &parser.current_token {
            Token::Plus => BinOp::Add,
            Token::Minus => BinOp::Sub,
            Token::ForwardSlash => BinOp::Div,
            Token::Asterisk => BinOp::Mul,
            Token::GreaterThan => BinOp::GreaterThan,
            Token::GreaterThanEqual => BinOp::GreaterThanEq,
            Token::LessThan => BinOp::LessThan,
            Token::LessThanEqual => BinOp::LessThanEq,
            Token::Equal => BinOp::Eq,
            Token::NotEqual => BinOp::NotEq,
            token => unimplemented!("Unexpected token {:?}", token),
        };

        parser.step();

        let right = P(Expr::parse(parser));

        ExprKind::Binary(operator, left, right)
    }

    /// Grammar: (operator) (expression)
    pub fn parse_unary(parser: &mut Parser) -> ExprKind {
        let operator = match &parser.current_token {
            Token::Minus => UnOp::Neg,
            token => unimplemented!("Unexpected token {:?}", token),
        };
        parser.step();

        println!("{:?}", parser.current_token);

        let expr = P(Expr::parse(parser));

        ExprKind::Unary(operator, expr)
    }

    /// Grammar: (identifier) => Token::Ident
    pub fn parse_ident(parser: &mut Parser) -> ExprKind {
        let symbol = match &parser.current_token {
            Token::Ident(value) => value.to_owned(),
            value => unimplemented!("Unexpected token {:?}", value),
        };

        if !parser.symbols.contains(&symbol) {
            panic!("symbol: {:?} reference before assignment", symbol);
        }

        ExprKind::Ident(symbol)
    }

    /// Grammar: (literal) => Token::Int | Token::String
    pub fn parse_literal(parser: &mut Parser) -> ExprKind {
        let litkind = match &parser.current_token {
            Token::String(value) => Literal::String(value.to_owned()),
            Token::Int(value) => Literal::Integer(value.to_owned()),
            value => unimplemented!("Unexpected token {:?}", value),
        };

        ExprKind::Literal(litkind)
    }
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
}

impl BinOp {
    pub fn token_match(token: &Token) -> bool {
        matches!(
            token,
            Token::Plus
                | Token::Minus
                | Token::Asterisk
                | Token::ForwardSlash
                | Token::Equal
                | Token::NotEqual
                | Token::LessThan
                | Token::LessThanEqual
                | Token::GreaterThan
                | Token::GreaterThanEqual
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum UnOp {
    //Not,
    Neg,
}

impl UnOp {
    pub fn token_match(token: &Token) -> bool {
        matches!(token, Token::Minus)
    }
}

pub type Ident = String;

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Integer(isize),
    // Add more
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{int_literal, lexer::Lexer, parser::P};

    #[test]
    fn identifier() {
        let mut lexer = Lexer::new("test".to_string());
        let mut parser = Parser::new(&mut lexer);

        parser.symbols.insert("test".to_owned());
        assert_eq!(
            Expr::parse_ident(&mut parser),
            ExprKind::Ident("test".to_owned())
        );
    }

    #[test]
    fn literal_int() {
        let mut lexer = Lexer::new("123".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(
            Expr::parse_literal(&mut parser),
            ExprKind::Literal(Literal::Integer(123))
        );
    }

    #[test]
    fn literal_string() {
        let mut lexer = Lexer::new("\"test\"".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(
            Expr::parse_literal(&mut parser),
            ExprKind::Literal(Literal::String("test".to_owned()))
        );
    }

    #[test]
    fn unary() {
        let mut lexer = Lexer::new("-123".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(
            Expr::parse_unary(&mut parser),
            ExprKind::Unary(UnOp::Neg, P(int_literal!(123)))
        );
    }

    #[test]
    fn binary() {
        let mut lexer = Lexer::new("123 * 456".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(
            Expr::parse_binary(&mut parser),
            ExprKind::Binary(BinOp::Mul, P(int_literal!(123)), P(int_literal!(456))),
        );
    }
}
