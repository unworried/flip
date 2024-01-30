use super::Expression;
use crate::parser::{Parse, Parser};

#[derive(Debug, PartialEq)]
pub struct Print {
    expression: Expression,
}

impl<'a> Parse<'a> for Print {
    fn parse(parser: &mut Parser<'a>) -> Self {
        parser.step();

        let expression = Expression::parse(parser);
        Self { expression }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{types::*, Statement},
        lexer::Lexer,
        parser::Parser,
    };

    use super::*;

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
}
