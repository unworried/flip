use super::{expression, Expr, Stmt};
use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

#[derive(Debug, PartialEq)]
pub struct Print {
    pub expression: Expr,
}

/// Grammar: "PRINT" (expression)
impl<'a> Parse<'a> for Print {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let expression = Expr::parse(parser);

        Self { expression }
    }
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub condition: Expr, // TODO: Implement conditions
    pub resolution: Vec<Stmt>,
}

/// Grammar: "IF" (condition) "THEN" \n {statement}* "ENDIF"
impl<'a> Parse<'a> for If {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let condition = Expr::parse(parser);

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
            resolution.push(Stmt::parse(parser));
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
    pub condition: Expr, // TODO: Impl Condtions
    pub resolution: Vec<Stmt>,
}

/// Grammar: "WHILE" (condition) "REPEAT" \n {statement}* "ENDWHILE"
impl<'a> Parse<'a> for While {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let condition = Expr::parse(parser);

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
            resolution.push(Stmt::parse(parser));
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

#[derive(Debug, PartialEq)]
pub struct Label {
    pub ident: expression::Ident,
}

/// Grammar: "LABEL" (ident)
impl<'a> Parse<'a> for Label {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let ident = expression::Ident::parse(parser);

        Self { ident }
    }
}

#[derive(Debug, PartialEq)]
pub struct Goto {
    pub ident: expression::Ident,
}

/// Grammar: "GOTO" (ident)
impl<'a> Parse<'a> for Goto {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let ident = expression::Ident::parse(parser);

        Self { ident }
    }
}

#[derive(Debug, PartialEq)]
pub struct Let {
    pub ident: expression::Ident,
    pub expression: Expr,
}

/// Grammar: "LET" (ident) "=" (expression)
impl<'a> Parse<'a> for Let {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let ident = expression::Ident::parse(parser);
        // TODO: May be able to move this to expr so wont need parser step here or into ident parse
        parser.step();

        if !parser.current_token(Token::Assign) {
            panic!("Expected EQUAL");
        }
        parser.step();

        let expression = Expr::parse(parser);

        Self { ident, expression }
    }
}

#[derive(Debug, PartialEq)]
pub struct Input {
    pub ident: expression::Ident,
}

/// Grammar: "INPUT" (ident)
impl<'a> Parse<'a> for Input {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let ident = expression::Ident::parse(parser);

        Self { ident }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Stmt, goto_stmt, if_stmt, int_primitive, label_stmt, let_stmt, lexer::Lexer,
        parser::Parser, print_stmt, string_literal, while_stmt,
    };

    fn check_abstract_tree(input: &str, expected: Vec<Stmt>) {
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
    fn while_statement() {
        let input = "WHILE \"TMP\" REPEAT\nPRINT \"hello, world!\" ENDWHILE";

        let expected = vec![while_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn while_statement_newline() {
        let input = "WHILE \"TMP\" REPEAT\nPRINT \"hello, world!\" ENDWHILE\n";

        let expected = vec![while_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn while_statement_nested_statements() {
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
    fn while_statement_nested_block_statements() {
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

    #[test]
    fn label_statement() {
        let input = "LABEL Ident";

        let expected = vec![label_stmt!("Ident".to_string())];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn label_statement_newline() {
        let input = "LABEL Ident\n";

        let expected = vec![label_stmt!("Ident".to_string())];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn goto_statement() {
        let input = "GOTO Ident";

        let expected = vec![goto_stmt!("Ident".to_string())];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn goto_statement_newline() {
        let input = "GOTO Ident\n";

        let expected = vec![goto_stmt!("Ident".to_string())];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn let_statement() {
        let input = "LET Ident = 123";

        let expected = vec![let_stmt!("Ident".to_string(), int_primitive!(123))];

        check_abstract_tree(input, expected)
    }
}
