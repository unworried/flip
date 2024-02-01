use super::{expression, Expr, Stmt, StmtKind};
use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

/// Grammar: "PRINT" (expression)
pub struct Print;
impl<'a> Parse<'a> for Print {
    type Item = StmtKind;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        let expression = Expr::parse(parser);

        StmtKind::Print(expression)
    }
}

/// Grammar: "IF" (condition) "THEN" \n {statement}* "ENDIF"
pub struct If;
impl<'a> Parse<'a> for If {
    type Item = StmtKind;

    fn parse(parser: &mut Parser<'a>) -> StmtKind {
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

        StmtKind::If(condition, resolution)
    }
}

/// Grammar: "WHILE" (condition) "REPEAT" \n {statement}* "ENDWHILE"
pub struct While;
impl<'a> Parse<'a> for While {
    type Item = StmtKind;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
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

        StmtKind::While(condition, resolution)
    }
}

/// Grammar: "LABEL" (ident)
pub struct Label;
impl<'a> Parse<'a> for Label {
    type Item = StmtKind;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        let ident = expression::Ident::parse(parser);

        StmtKind::Label(ident)
    }
}

/// Grammar: "GOTO" (ident)
pub struct Goto;
impl<'a> Parse<'a> for Goto {
    type Item = StmtKind;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        let ident = expression::Ident::parse(parser);

        StmtKind::Goto(ident)
    }
}

/// Grammar: "LET" (ident) "=" (expression)
pub struct Let;
impl<'a> Parse<'a> for Let {
    type Item = StmtKind;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        let ident = expression::Ident::parse(parser);
        // TODO: May be able to move this to expr so wont need parser step here or into ident parse
        parser.step();

        if !parser.current_token(Token::Assign) {
            panic!("Expected EQUAL");
        }
        parser.step();

        let expression = Expr::parse(parser);

        StmtKind::Let(ident, expression)
    }
}

/// Grammar: "INPUT" (ident)
pub struct Input;
impl<'a> Parse<'a> for Input {
    type Item = StmtKind;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        let ident = expression::Ident::parse(parser);

        StmtKind::Input(ident)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Stmt, goto_stmt, if_stmt, int_literal, label_stmt, let_stmt, lexer::Lexer,
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

        let expected = vec![print_stmt!(int_literal!(123))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn print_int_statement_newline() {
        let input = "PRINT 123\n";

        let expected = vec![print_stmt!(int_literal!(123))];

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

        let expected = vec![let_stmt!("Ident".to_string(), int_literal!(123))];

        check_abstract_tree(input, expected)
    }
}
