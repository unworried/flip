use super::{Expression, Statement};
use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

#[derive(Debug, PartialEq)]
pub struct Print {
    pub expression: Expression,
}

impl<'a> Parse<'a> for Print {
    fn parse(parser: &mut Parser<'a>) -> Self {
        parser.step();

        let expression = Expression::parse(parser);
        Self { expression }
    }
}

#[derive(Debug, PartialEq)]
pub struct Conditional {
    pub condition: Expression, // TODO: Implement conditions
    pub resolution: Vec<Statement>,
}

impl<'a> Parse<'a> for Conditional {
    fn parse(parser: &mut Parser<'a>) -> Self {
        parser.step();

        let condition = Expression::parse(parser);

        if !parser.current_token(Token::Then) {
            panic!("Expected THEN");
        }
        parser.step();

        if !parser.current_token(Token::Newline) {
            panic!("Expected newline");
        }
        parser.step();

        let mut resolution = Vec::new();
        while !parser.current_token(Token::EndIf) {
            resolution.push(Statement::parse(parser));
        }

        if !parser.current_token(Token::EndIf) {
            panic!("Expected ENDIF");
        }
        parser.step();

        Self {
            condition,
            resolution,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{expression::Literal, statement::*};
    use crate::{ast::Statement, lexer::Lexer, parser::Parser};

    fn check_abstract_tree(input: &str, expected: Statement) {
        let mut lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lex);
        let result = Statement::parse(&mut parser);

        assert_eq!(result, expected);
    }

    #[test]
    fn print_statement() {
        let input = "PRINT \"hello, world!\"";

        let expected = Statement::Print(Print {
            expression: Expression::Literal(Literal::String("hello, world!".to_string())),
        });

        check_abstract_tree(input, expected)
    }


    #[test]
    fn print_statement_newline() {
        let input = "PRINT \"hello, world!\"\n";

        let expected = Statement::Print(Print {
            expression: Expression::Literal(Literal::String("hello, world!".to_string())),
        });

        check_abstract_tree(input, expected)
    }

    #[test]
    fn conditional_statement() {
        let input = "IF \"TMP\" THEN\nPRINT \"hello, world!\" ENDIF\n";

        let expected = Statement::Conditional(Conditional {
            condition: Expression::Literal(Literal::String("TMP".to_string())),
            resolution: vec![Statement::Print(Print {
                expression: Expression::Literal(Literal::String("hello, world!".to_string())),
            })],
        });

        check_abstract_tree(input, expected)
    }
}
