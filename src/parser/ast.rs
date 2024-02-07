use crate::lexer::Token;
use crate::parser::P;
use crate::resolver::DefinitionId;
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
            id: None,
            pattern,
            value: P(value),
            span,
        })
    }

    pub fn assignment(pattern: Pattern, value: Ast, span: Span) -> Ast {
        Ast::Assignment(Definition {
            id: None,
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
            definition: None,
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
    pub id: Option<DefinitionId>,
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
    pub definition: Option<DefinitionId>,
    pub span: Span,
}

pub type Ident = String;

/*#[derive(Debug)]
pub struct Ast {
    pub items: Vec<Item>, // HashMap<ItemIdm, Item>
}

/// Grammar: {(statement);}*
impl Ast {
    pub fn parse(parser: &mut Parser, end_delim: Token) -> Self {
        let mut items = Vec::new();
        while !parser.current_token_is(&end_delim) {
            items.push(Item::parse(parser));

            while parser.current_token_is(&Token::Newline) {
                parser.step();
            }
        }

        Self { items }
    }
}

#[derive(Debug)]
pub struct Item {
    pub kind: ItemKind,
}

#[derive(Debug)]
pub enum ItemKind {
    //Function(Function),
    Statement(Stmt),
}

impl<'a> Parse<'a> for Item {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let kind = ItemKind::Statement(Stmt::parse(parser));

        Self { kind }
    }
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum StmtKind {
    // "let" (identifier) "=" (expression)
    Let(P<Local>), // Fix this
    // (variable) "=" (expression)
    Assignment(P<Local>),
    // "if" (condition) "{" \n {statement}* "}"
    If(Expr, Vec<Item>), // WARN: When funcs are added. need to change this to only allow stmts
    // "while" (condition) "{" \n {statement}* "}"
    While(Expr, Vec<Item>),
    Error,
}

impl<'a> Parse<'a> for Stmt {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let (token, start_span) = parser.consume();

        let kind = match &token {
            Token::Let => Self::parse_let(parser),
            Token::Ident(ident) => {
                Self::parse_assignment(parser, (ident.to_owned(), start_span.clone()))
            }
            Token::If => Self::parse_if(parser),
            Token::While => Self::parse_while(parser),
            token => {
                parser
                    .diagnostics
                    .borrow_mut()
                    .unexpected_statement(token, &start_span);
                StmtKind::Error
            } // Handle Err
        };

        parser.consume_and_check(Token::SemiColon);

        Self {
            kind,
            span: Span::combine(vec![&start_span, &parser.current_span()]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Binary(BinOp, P<Expr>, P<Expr>),
    Unary(UnOp, P<Expr>),
    Literal(expression::Literal),
    Variable(Ident),
    Error,
}

impl<'a> Parse<'a> for Expr {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let start_span = parser.current_span().clone();

        let mut kind = Self::parse_unary_or_primary(parser);

        if BinOp::token_match(parser.current_token()) {
            kind = Self::parse_binary(parser, kind, 0);
        }

        Expr {
            kind,
            span: Span::combine(vec![&start_span, &parser.current_span()]),
        }
    }
}*/
