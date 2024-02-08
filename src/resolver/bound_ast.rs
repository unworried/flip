use crate::parser::ast::{Ast, Binary, If, Literal, Unary, While};
use crate::parser::P;

pub enum BoundAst {
    Definition(Definition),
    Variable(Variable),

    Sequence(Sequence),
    If(If),
    While(While),
    Binary(Binary),
    Unary(Unary),
    Literal(Literal),
    Error,
}

pub struct Sequence {
    pub statements: Vec<BoundAst>,
}

pub struct Definition {
    pub id: usize,
    pub pattern: Pattern,
    pub value: P<BoundAst>,
    pub uses: usize,
}

pub struct Variable {
    pub pattern: String,
    pub definition: usize,
}

pub struct Pattern {
    pub name: String,
}

impl Into<BoundAst> for Ast {
    fn into(self) -> BoundAst {
        match self {
            Ast::Sequence(seq) => BoundAst::Sequence(Sequence {
                statements: seq.statements.into_iter().map(|ast| ast.into()).collect(),
            }),
            Ast::Let(def) | Ast::Assignment(def) => BoundAst::Definition(Definition {
                id: 0,
                pattern: Pattern {
                    name: def.pattern.name,
                },
                value: P(def.value.into_inner().into()),
                uses: 0,
            }),
            Ast::Variable(var) => BoundAst::Variable(Variable {
                pattern: var.pattern,
                definition: 0,
            }),
            Ast::Binary(bin) => BoundAst::Binary(bin),
            Ast::Unary(un) => BoundAst::Unary(un),
            Ast::If(if_expr) => BoundAst::If(if_expr),
            Ast::While(while_expr) => BoundAst::While(while_expr),
            Ast::Literal(lit) => BoundAst::Literal(lit),
            Ast::Error => BoundAst::Error,
        }
    }
}
