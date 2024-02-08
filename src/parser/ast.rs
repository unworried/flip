use crate::lexer::Token;
use crate::parser::P;
use crate::span::Span;

#[derive(Debug, Clone)]
pub enum Ast {
    Sequence(Sequence),
    Let(Definition), // Fix this
    Assignment(Definition),
    If(If), // WARN: When funcs are added. need to change this to only allow stmts
    While(While),
    Binary(Binary),
    Unary(Unary),
    Literal(Literal),
    Variable(Variable),
    Error,
}

impl Ast {
    pub fn sequence(statements: Vec<Ast>, span: Span) -> Ast {
        Ast::Sequence(Sequence { statements, span })
    }

    pub fn definition(pattern: Pattern, value: Ast, span: Span) -> Ast {
        Ast::Let(Definition {
            pattern,
            value: P(value),
            span,
        })
    }

    pub fn assignment(pattern: Pattern, value: Ast, span: Span) -> Ast {
        Ast::Assignment(Definition {
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

    pub fn variable(pattern: Ident, span: Span) -> Ast {
        Ast::Variable(Variable {
            pattern,
            span,
        })
    }
}
#[derive(Debug, Clone)]
pub struct Sequence {
    pub statements: Vec<Ast>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: Ident,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Definition {
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
    String(String),
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub pattern: Ident,
    pub span: Span,
}

pub type Ident = String;

