use std::hash::{Hash, Hasher};

use crate::ast::ptr::P;
use crate::lexer::Token;
use crate::span::Span;

mod display;
pub mod ptr;
pub mod visitor;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub pattern: Pattern,
    pub parameters: Vec<Pattern>,
    pub body: Ast,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Ast {
    Sequence(Sequence),
    Definition(Definition),
    Assignment(Assignment),
    Return(P<Ast>),
    If(If), // WARN: When funcs are added. need to change this to only allow stmts
    While(While),
    Binary(Binary),
    Unary(Unary),
    Literal(Literal),
    Variable(Variable),
    Call(Call),
    Error,
}

impl Ast {
    pub fn sequence(expressions: Vec<Ast>, span: Span) -> Ast {
        Ast::Sequence(Sequence { expressions, span })
    }

    pub fn definition(pattern: Pattern, value: Ast, span: Span) -> Ast {
        Ast::Definition(Definition {
            pattern,
            value: P(value),
            span,
        })
    }

    pub fn assignment(pattern: Pattern, value: Ast, span: Span) -> Ast {
        Ast::Assignment(Assignment {
            pattern,
            value: P(value),
            span,
        })
    }

    pub fn if_expr(condition: Ast, then: Ast, span: Span) -> Ast {
        Ast::If(If {
            condition: P(condition),
            then: P(then),
            span,
        })
    }

    pub fn while_expr(condition: Ast, then: Ast, span: Span) -> Ast {
        Ast::While(While {
            condition: P(condition),
            then: P(then),
            span,
        })
    }

    pub fn integer(value: u64, span: Span) -> Ast {
        Ast::Literal(Literal {
            kind: LiteralKind::Int(value),
            span,
        })
    }

    pub fn char(value: char, span: Span) -> Ast {
        Ast::Literal(Literal {
            kind: LiteralKind::Char(value),
            span,
        })
    }

    pub fn string(value: String, span: Span) -> Ast {
        Ast::Literal(Literal {
            kind: LiteralKind::String(value),
            span,
        })
    }

    pub fn binary(op: BinOp, left: Ast, right: Ast, span: Span) -> Ast {
        Ast::Binary(Binary {
            op,
            left: P(left),
            right: P(right),
            span,
        })
    }

    pub fn unary(op: UnOp, oprand: Ast, span: Span) -> Ast {
        Ast::Unary(Unary {
            op,
            operand: P(oprand),
            span,
        })
    }

    pub fn call(pattern: Pattern, arguments: Vec<Ast>, span: Span) -> Ast {
        Ast::Call(Call {
            pattern,
            arguments,
            span,
        })
    }

    pub fn variable(name: Ident, span: Span) -> Ast {
        Ast::Variable(Variable { name, span })
    }

    pub fn return_expr(value: Ast) -> Ast {
        Ast::Return(P(value))
    }
}
#[derive(Debug, Clone)]
pub struct Sequence {
    pub expressions: Vec<Ast>,
    pub span: Span,
}

#[derive(Debug, Clone, Eq)]
pub struct Pattern {
    pub name: Ident,
    pub span: Span,
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Pattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Clone)] // Convert to unbound and bound trees instead
pub struct Definition {
    pub pattern: Pattern,
    pub value: P<Ast>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub pattern: Pattern,
    pub value: P<Ast>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: P<Ast>,
    pub then: P<Ast>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: P<Ast>,
    pub then: P<Ast>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub op: BinOp,
    pub left: P<Ast>,
    pub right: P<Ast>,
    pub span: Span,
}

#[derive(Debug, Clone)]
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

    pub fn precedence(&self) -> u8 {
        match self {
            BinOp::Add | BinOp::Sub => 18,
            BinOp::Mul | BinOp::Div => 19,
            BinOp::Eq | BinOp::NotEq => 30,
            BinOp::LessThan | BinOp::LessThanEq | BinOp::GreaterThan | BinOp::GreaterThanEq => 29,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub op: UnOp,
    pub operand: P<Ast>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum UnOp {
    //Not,
    Neg,
}

impl UnOp {
    pub fn token_match(token: &Token) -> bool {
        matches!(token, Token::Minus)
    }
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Int(u64),
    Char(char),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Call {
    pub pattern: Pattern,
    pub arguments: Vec<Ast>,
    pub span: Span,
}

pub type Variable = Pattern;

pub type Ident = String;
