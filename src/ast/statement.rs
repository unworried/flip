use super::{expression, Expression, Statement};
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
        //parser.step(); WARN: CHANGEGP1

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
        //parser.step(); WARN: CHANGEGP1

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
pub struct While {
    pub condition: Expression, // TODO: Impl Condtions
    pub resolution: Vec<Statement>,
}

impl<'a> Parse<'a> for While {
    fn parse(parser: &mut Parser<'a>) -> Self {
        //parser.step(); WARN: CHANGEGP1

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

pub struct Label {
    pub ident: expression::Identifier,
}

impl<'a> Parse<'a> for Label {
    fn parse(parser: &mut Parser<'a>) -> Self {
        parser.step();

        let ident = expression::Identifier::parse(parser);

        Self { ident }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Statement, if_stmt, int_primitive, lexer::Lexer, while_stmt, parser::Parser,
        print_stmt, string_literal,
    };

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
    fn print_string_statement() {
        let input = "PRINT \"hello, world!\"";

        let expected = vec![print_stmt!(string_literal!("hello, world!"))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn print_string_statement_newline() {
        let input = "PRINT \"hello, world!\"\n";

        let expected = vec![print_stmt!(string_literal!("hello, world!"))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn print_int_statement() {
        let input = "PRINT 123";

        let expected = vec![print_stmt!(int_primitive!(123))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn print_int_statement_newline() {
        let input = "PRINT 123\n";

        let expected = vec![print_stmt!(int_primitive!(123))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn if_statement() {
        let input = "IF \"TMP\" THEN\nPRINT \"hello, world!\" ENDIF";

        let expected = vec![if_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn if_statement_newline() {
        let input = "IF \"TMP\" THEN\nPRINT \"hello, world!\" ENDIF\n";

        let expected = vec![if_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn loop_statement() {
        let input = "WHILE \"TMP\" REPEAT\nPRINT \"hello, world!\" ENDWHILE";

        let expected = vec![while_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn loop_statement_newline() {
        let input = "WHILE \"TMP\" REPEAT\nPRINT \"hello, world!\" ENDWHILE\n";

        let expected = vec![while_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn loop_statement_nested_statements() {
        let input = "WHILE \"TMP\" REPEAT\nPRINT \"hello, world!\"\nPRINT \"hello, world 2!\"\nPRINT \"hello, world 3!\"\nENDWHILE";

        let expected = vec![while_stmt!(
            string_literal!("TMP"),
            vec![
                print_stmt!(string_literal!("hello, world!")),
                print_stmt!(string_literal!("hello, world 2!")),
                print_stmt!(string_literal!("hello, world 3!")),
            ]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn loop_statement_nested_block_statements() {
        let input = "WHILE \"TMP\" REPEAT\nPRINT \"hello, world!\"\nIF \"TMP\" THEN\nWHILE \"TMP\" REPEAT\nPRINT \"hello, world 3!\"\nENDWHILE\nENDIF\nENDWHILE";

        let expected = vec![while_stmt!(
            string_literal!("TMP"),
            vec![
                print_stmt!(string_literal!("hello, world!")),
                if_stmt!(
                    string_literal!("TMP"),
                    vec![while_stmt!(
                        string_literal!("TMP"),
                        vec![print_stmt!(string_literal!("hello, world 3!"))]
                    )]
                )
            ]
        )];

        check_abstract_tree(input, expected)
    }
}
