use std::cmp;

use crate::ast::{Ast, BinOp, Function, Pattern, Program, UnOp};
use crate::lexer::Token;
use crate::parser::Parser;
use crate::span::Span;

pub fn parse_program(parser: &mut Parser) -> Program {
    let mut functions = Vec::new();
    while !parser.current_token_is(&Token::Eof) {
        while parser.current_token_is(&Token::Newline) {
            parser.step();
        }

        functions.push(parse_function(parser));

        while parser.current_token_is(&Token::Newline) {
            parser.step();
        }
    }

    Program { functions }
}

pub fn parse_function(parser: &mut Parser) -> Function {
    if let (Token::Ident(name), span) = parser.consume() {
        // Parameters
        parser.expect(Token::LParen);
        let parameters = parse_parameters(parser);
        parser.expect(Token::RParen);

        // Body Block
        parser.expect(Token::LBrace);
        let body = parse_sequence(parser, Token::RBrace);
        parser.expect(Token::RBrace);

        let pattern = Pattern {
            name: name.to_owned(),
            span,
        };

        // TODO: Test this - Allows optional `;` at end of function
        parser.optional(Token::SemiColon);

        return Function {
            pattern,
            parameters,
            body,
            span: Span::combine(vec![&span, &parser.current_span()]),
        };
    }
    // FIXME: ...
    panic!("Implement error handling: {}", parser.current_token());
}

// TODO: Test
fn parse_parameters(parser: &mut Parser) -> Vec<Pattern> {
    let mut parameters: Vec<Pattern> = Vec::new();
    while !parser.current_token_is(&Token::RParen) {
        let (token, param_span) = parser.consume();
        match token {
            Token::Ident(name) => {
                parameters.push(Pattern {
                    name: name.to_owned(),
                    span: param_span,
                });

                if !parser.current_token_is(&Token::RParen) {
                    parser.expect(Token::Comma);
                }
            }
            _ => {
                parser
                    .diagnostics
                    .borrow_mut()
                    .unexpected_token(&token, &param_span);
                parser.step_until(&Token::RParen);
                break;
            }
        }
    }
    parameters
}

pub fn parse_sequence(parser: &mut Parser, end_delim: Token) -> Ast {
    let start_span = parser.current_span();
    let mut statements = Vec::new();
    while !parser.current_token_is(&end_delim) && !parser.current_token_is(&Token::Eof) {
        while parser.current_token_is(&Token::Newline) {
            parser.step();
        }

        statements.push(parse_statement(parser));

        while parser.current_token_is(&Token::Newline) {
            parser.step();
        }
    }

    let span = Span::combine(vec![&start_span, &parser.current_span()]);
    if statements.is_empty() {
        parser.diagnostics.borrow_mut().empty_block(&span);
    }

    Ast::sequence(statements, span)
}

pub fn parse_statement(parser: &mut Parser) -> Ast {
    let (token, span) = parser.consume();

    let stmt = match &token {
        Token::Let => parse_let(parser),
        Token::Ident(name) => {
            let pattern = Pattern {
                name: name.to_owned(),
                span,
            };
            parse_assignment_or_call(parser, pattern)
        }
        Token::If => parse_if(parser),
        Token::While => parse_while(parser),
        _ => {
            parser
                .diagnostics
                .borrow_mut()
                .unknown_statement(&token, &span);
            Ast::Error
        } // Handle Err
    };

    parser.expect(Token::SemiColon);

    stmt
}

pub fn parse_expression(parser: &mut Parser) -> Ast {
    let mut expr = parse_unary_or_primary(parser);

    if BinOp::token_match(parser.current_token()) {
        expr = parse_binary(parser, expr, 0);
    }

    expr
}

pub fn parse_let(parser: &mut Parser) -> Ast {
    //let ident = Ident::parse(parser);
    // Temp solution to seperate assignment from refernece. do this properly later...
    let start_span = parser.current_span();
    let name = match &parser.current_token() {
        Token::Ident(value) => value.to_owned(),
        value => {
            parser
                .diagnostics
                .borrow_mut()
                .unexpected_token(value, &start_span);
            return Ast::Error;
        }
    };
    parser.step();

    parser.expect(Token::Assign);

    let pattern = Pattern {
        name,
        span: start_span,
    };

    let value = parse_expression(parser);

    Ast::definition(
        pattern,
        value,
        Span::combine(vec![&start_span, &parser.current_span()]),
    )
}

pub fn parse_assignment_or_call(parser: &mut Parser, pattern: Pattern) -> Ast {
    let (token, span) = parser.consume();

    match token {
        Token::Assign => parse_assignment(parser, pattern),
        Token::LParen => {
            let args = parse_arguments(parser);
            parser.expect(Token::RParen);
            Ast::call(
                pattern,
                args,
                Span::combine(vec![&span, &parser.current_span()]),
            )
        }
        _ => {
            parser.step_until(&Token::SemiColon);
            // TODO: Add Err?
            Ast::Error
        }
    }
}

// TODO: Test
fn parse_arguments(parser: &mut Parser) -> Vec<Ast> {
    let mut args: Vec<Ast> = Vec::new();
    while !parser.current_token_is(&Token::RParen) {
        args.push(parse_expression(parser));

        if !parser.current_token_is(&Token::RParen) {
            parser.expect(Token::Comma);
        }
    }

    args
}

pub fn parse_assignment(parser: &mut Parser, pattern: Pattern) -> Ast {
    let start_span = pattern.span;
    /*let start_span = parser.current_span();
    if !parser.expect_with_outcome(Token::Assign) {
        parser.step_until(&Token::SemiColon);
        return Ast::Error;
    }*/

    let value = parse_expression(parser);

    Ast::assignment(
        pattern,
        value,
        Span::combine(vec![&start_span, &parser.current_span()]),
    )
}

pub fn parse_if(parser: &mut Parser) -> Ast {
    let start_span = parser.current_span();
    let condition = parse_expression(parser);

    parser.expect(Token::LBrace);

    // Newline is optional May not need if allow newlines at start of file in ast root struct
    while parser.current_token_is(&Token::Newline) {
        parser.step();
    }

    let resolution = parse_sequence(parser, Token::RBrace);

    parser.expect(Token::RBrace);

    Ast::if_expr(
        condition,
        resolution,
        Span::combine(vec![&start_span, &parser.current_span()]),
    )
}

pub fn parse_while(parser: &mut Parser) -> Ast {
    let start_span = parser.current_span();
    let condition = parse_expression(parser);

    parser.expect(Token::LBrace);

    // Newline is optional May not need if allow newlines at start of file in ast root struct
    while parser.current_token_is(&Token::Newline) {
        parser.step();
    }

    let resolution = parse_sequence(parser, Token::RBrace);

    parser.expect(Token::RBrace);

    Ast::while_expr(
        condition,
        resolution,
        Span::combine(vec![&start_span, &parser.current_span()]),
    )
}

pub fn parse_unary_or_primary(parser: &mut Parser) -> Ast {
    if UnOp::token_match(parser.current_token()) {
        parse_unary(parser)
    } else {
        parse_primary(parser)
    }
}

pub fn parse_unary(parser: &mut Parser) -> Ast {
    let start_span = parser.current_span();
    let operator = match &parser.current_token() {
        Token::Minus => UnOp::Neg,
        _ => return Ast::Error,
    };

    /* FIXME:
     * Should this really be caught here?
     * Catches cases where whitespace between operator and expression
     * e.g. - 1, let foo = - bar;
     * instead of:
     * -1, let foo = -bar;
     */
    parser.expect_flush();

    let operand = parse_unary_or_primary(parser);
    Ast::unary(
        operator,
        operand,
        Span::combine(vec![&start_span, &parser.current_span()]),
    )
}

pub fn parse_primary(parser: &mut Parser) -> Ast {
    let (token, span) = parser.consume();

    match &token {
        // Temp before i split into parse_int and parse string
        Token::Int(value) => Ast::integer(value.to_owned(), span),
        Token::String(value) => Ast::string(value.to_owned(), span),
        Token::LParen => parse_group(parser),
        // Grammar: (identifier) => Token::Ident
        Token::Ident(symbol) => Ast::variable(symbol.to_owned(), span),
        _ => panic!("Really shouldn't reach here, implement fatal error instead"),
    }
}

pub fn parse_group(parser: &mut Parser) -> Ast {
    let expr = parse_expression(parser);
    parser.expect(Token::RParen);

    expr
}

pub fn parse_binary(parser: &mut Parser, mut left: Ast, precedence: u8) -> Ast {
    let start_span = parser.current_span();
    while let Some(operator) = parse_binary_operator(parser) {
        if operator.precedence() < precedence {
            break;
        }
        parser.step();

        let mut right = parse_unary_or_primary(parser);

        while let Some(inner_operator) = parse_binary_operator(parser) {
            let greater_precedence = inner_operator.precedence() > operator.precedence();
            let equal_precedence = inner_operator.precedence() == operator.precedence();
            if !greater_precedence && !equal_precedence {
                break;
            }

            right = parse_binary(
                parser,
                right,
                cmp::max(operator.precedence(), inner_operator.precedence()),
            );
        }
        left = Ast::binary(
            operator,
            left,
            right,
            Span::combine(vec![&start_span, &parser.current_span()]),
        );
    }
    left
}

fn parse_binary_operator(parser: &mut Parser) -> Option<BinOp> {
    match &parser.current_token() {
        Token::Plus => Some(BinOp::Add),
        Token::Minus => Some(BinOp::Sub),
        Token::ForwardSlash => Some(BinOp::Div),
        Token::Asterisk => Some(BinOp::Mul),
        Token::GreaterThan => Some(BinOp::GreaterThan),
        Token::GreaterThanEqual => Some(BinOp::GreaterThanEq),
        Token::LessThan => Some(BinOp::LessThan),
        Token::LessThanEqual => Some(BinOp::LessThanEq),
        Token::Equal => Some(BinOp::Eq),
        Token::NotEqual => Some(BinOp::NotEq),
        _ => None,
    }
}
