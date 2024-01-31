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
pub struct If {
    pub condition: Expression, // TODO: Implement conditions
    pub resolution: Vec<Statement>,
}

impl<'a> Parse<'a> for If {
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

#[derive(Debug, PartialEq)]
pub struct Loop {
    pub condition: Expression, // TODO: Impl Condtions
    pub resolution: Vec<Statement>,
}

impl<'a> Parse<'a> for Loop {
    fn parse(parser: &mut Parser<'a>) -> Self {
        parser.step();

        let condition = Expression::parse(parser);

        if !parser.current_token(Token::Repeat) {
            panic!("Expected REPEAT");
        }
        parser.step();

        if !parser.current_token(Token::Newline) {
            panic!("Expected newline");
        }
        parser.step();

        let mut resolution = Vec::new();
        while !parser.current_token(Token::EndWhile) {
            resolution.push(Statement::parse(parser));
        }

        if !parser.current_token(Token::EndWhile) {
            panic!("Expected ENDWHILE");
        }
        parser.step();

        Self {
            condition,
            resolution,
        }
    }
}

/*pub struct Label {
    pub name: expression::Identifier,
}

impl<'a> Parse<'a> for Label {
    fn parse(parser: &mut Parser<'a>) -> Self {
        parser.step();

        let name = expression::Identifier::parse(parser);

        Self { name }
    }
}*/
// TODO: Come back to

#[cfg(test)]
mod tests {
    use crate::ast::{expression::Literal, statement::*};
    use crate::{ast::Statement, lexer::Lexer, parser::Parser};

    fn check_abstract_tree(input: &str, expected: Vec<Statement>) {
        let mut lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lex);
        let result = parser.parse();

        println!("{:#?}", result.statements);
        println!();
        println!("{:#?}", expected);
        assert_eq!(result.statements, expected);
    }

    #[test]
    fn print_statement() {
        let input = "PRINT \"hello, world!\"";

        let expected = vec![Statement::Print(Print {
            expression: Expression::Literal(Literal::String("hello, world!".to_string())),
        })];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn print_statement_newline() {
        let input = "PRINT \"hello, world!\"\n";

        let expected = vec![Statement::Print(Print {
            expression: Expression::Literal(Literal::String("hello, world!".to_string())),
        })];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn if_statement() {
        let input = "IF \"TMP\" THEN\nPRINT \"hello, world!\" ENDIF";

        let expected = vec![Statement::If(If {
            condition: Expression::Literal(Literal::String("TMP".to_string())),
            resolution: vec![Statement::Print(Print {
                expression: Expression::Literal(Literal::String("hello, world!".to_string())),
            })],
        })];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn if_statement_newline() {
        let input = "IF \"TMP\" THEN\nPRINT \"hello, world!\" ENDIF\n";

        let expected = vec![Statement::If(If {
            condition: Expression::Literal(Literal::String("TMP".to_string())),
            resolution: vec![Statement::Print(Print {
                expression: Expression::Literal(Literal::String("hello, world!".to_string())),
            })],
        })];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn loop_statement() {
        let input = "WHILE \"TMP\" REPEAT\nPRINT \"hello, world!\" ENDWHILE";

        let expected = vec![Statement::Loop(Loop {
            condition: Expression::Literal(Literal::String("TMP".to_string())),
            resolution: vec![Statement::Print(Print {
                expression: Expression::Literal(Literal::String("hello, world!".to_string())),
            })],
        })];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn loop_statement_newline() {
        let input = "WHILE \"TMP\" REPEAT\nPRINT \"hello, world!\" ENDWHILE\n";

        let expected = vec![Statement::Loop(Loop {
            condition: Expression::Literal(Literal::String("TMP".to_string())),
            resolution: vec![Statement::Print(Print {
                expression: Expression::Literal(Literal::String("hello, world!".to_string())),
            })],
        })];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn loop_statement_nested_statements() {
        let input = "WHILE \"TMP\" REPEAT\nPRINT \"hello, world!\"\nPRINT \"hello, world 2!\"\nPRINT \"hello, world 3!\"\nENDWHILE";

        let expected = vec![Statement::Loop(Loop {
            condition: Expression::Literal(Literal::String("TMP".to_string())),
            resolution: vec![
                Statement::Print(Print {
                    expression: Expression::Literal(Literal::String("hello, world!".to_string())),
                }),
                Statement::Print(Print {
                    expression: Expression::Literal(Literal::String("hello, world 2!".to_string())),
                }),
                Statement::Print(Print {
                    expression: Expression::Literal(Literal::String("hello, world 3!".to_string())),
                }),
            ],
        })];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn loop_statement_nested_block_statements() {
        let input = "WHILE \"TMP\" REPEAT\nPRINT \"hello, world!\"\nIF \"TMP\" THEN\nWHILE \"TMP\" REPEAT\nPRINT \"hello, world 3!\"\nENDWHILE\nENDIF\nENDWHILE";

        // Help me
        let expected = vec![Statement::Loop(Loop {
            condition: Expression::Literal(Literal::String("TMP".to_string())),
            resolution: vec![
                Statement::Print(Print {
                    expression: Expression::Literal(Literal::String("hello, world!".to_string())),
                }),
                Statement::If(If {
                    condition: Expression::Literal(Literal::String("TMP".to_string())),
                    resolution: vec![Statement::Loop(Loop {
                        condition: Expression::Literal(Literal::String("TMP".to_string())),
                        resolution: vec![Statement::Print(Print {
                            expression: Expression::Literal(Literal::String(
                                "hello, world 3!".to_string(),
                            )),
                        })],
                    })],
                }),
            ],
        })];

        check_abstract_tree(input, expected)
    }
}
