use super::{Ast, Expr, Stmt, StmtKind};
use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

impl Stmt {
    /// Grammar: "PRINT" (expression)
    pub fn parse_print(parser: &mut Parser) -> StmtKind {
        let expression = Expr::parse(parser);

        StmtKind::Print(expression)
    }

    /// Grammar: "IF" (condition) "THEN" \n {statement}* "ENDIF"
    pub fn parse_if(parser: &mut Parser) -> StmtKind {
        let condition = Expr::parse(parser);

        if !parser.current_token(&Token::Then) {
            panic!("expected: THEN, actual: {:?}", parser.current_token);
        }
        parser.step();

        if !parser.current_token(&Token::Newline) {
            panic!("expected: newline, actual: {:?}", parser.current_token);
        }
        parser.step();

        let resolution = Ast::parse(parser, Token::EndIf).items;

        if !parser.current_token(&Token::EndIf) {
            panic!("expected: ENDIF, actual: {:?}", parser.current_token);
        }
        parser.step();

        StmtKind::If(condition, resolution)
    }

    /// Grammar: "WHILE" (condition) "REPEAT" \n {statement}* "ENDWHILE"
    pub fn parse_while(parser: &mut Parser) -> StmtKind {
        let condition = Expr::parse(parser);

        if !parser.current_token(&Token::Repeat) {
            panic!("expected: REPEAT, actual: {:?}", parser.current_token);
        }
        parser.step();

        if !parser.current_token(&Token::Newline) {
            panic!("expected: newline, actual: {:?}", parser.current_token);
        }
        parser.step();

        let resolution = Ast::parse(parser, Token::EndWhile).items;

        if !parser.current_token(&Token::EndWhile) {
            panic!("expected: ENDWHILE, actual: {:?}", parser.current_token);
        }
        parser.step();

        StmtKind::While(condition, resolution)
    }

    /// Grammar: "LET" (ident) "=" (expression)
    pub fn parse_let(parser: &mut Parser) -> StmtKind {
        //let ident = Ident::parse(parser);
        // Temp solution to seperate assignment from refernece. do this properly later...
        let ident = match &parser.current_token {
            Token::Ident(value) => value.to_owned(),
            value => unimplemented!("Unexpected token {:?}", value),
        };

        parser.step();

        if !parser.current_token(&Token::Assign) {
            panic!("expected: Assignment, actual: {:?}", parser.current_token);
        }
        parser.step();

        let expression = Expr::parse(parser);

        if !parser.symbols.insert(ident.to_owned()) {
            // Should this be to_owned??
            panic!("symbol: {:?} already defined", ident);
        }

        StmtKind::Let(ident, expression)
    }
}

/*/// Grammar: "LABEL" (ident)
pub struct Label;
impl<'a> Parse<'a> for Label {
    type Item = StmtKind;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        let ident = Ident::parse(parser);
        parser.step();

        StmtKind::Label(ident)
    }
}*/

/*/// Grammar: "GOTO" (ident)
pub struct Goto;
impl<'a> Parse<'a> for Goto {
    type Item = StmtKind;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        let ident = Ident::parse(parser);
        parser.step();

        StmtKind::Goto(ident)
    }
}*/

/*/// Grammar: "INPUT" (ident)
pub struct Input;
impl<'a> Parse<'a> for Input {
    type Item = StmtKind;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        // Temp solution to seperate assignment from refernece. do this properly later...
        let ident = match &parser.current_token {
            Token::Ident(value) => value.to_owned(),
            value => unimplemented!("Unexpected token {:?}", value),
        };

        if !parser.symbols.insert(ident.to_owned()) {
            // Should this be to_owned??
            panic!("symbol: {:?} already defined", ident);
        }

        parser.step();

        StmtKind::Input(ident)
    }
}*/

#[cfg(test)]
mod tests {
    use crate::{
        ast::Item, if_stmt, int_literal, let_stmt, lexer::Lexer, parser::Parser, print_stmt,
        string_literal, while_stmt,
    };

    fn check_abstract_tree(input: &str, expected: Vec<Item>) {
        let mut lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lex);
        let result = parser.parse();

        println!("{:#?}", result.items);
        println!();
        println!("{:#?}", expected);
        assert_eq!(result.items, expected);
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
    fn let_statement() {
        let input = "LET Ident = 123";

        let expected = vec![let_stmt!("Ident".to_string(), int_literal!(123))];

        check_abstract_tree(input, expected)
    }
}
