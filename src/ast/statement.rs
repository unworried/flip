use super::{Ast, Expr, Stmt, StmtKind};
use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

impl Stmt {
    /// Grammar: "print" (expression)
    pub fn parse_print(parser: &mut Parser) -> StmtKind {
        let expression = Expr::parse(parser);

        StmtKind::Print(expression)
    }

    /// Grammar: "if" (condition) "then" \n {statement}* "endif"
    pub fn parse_if(parser: &mut Parser) -> StmtKind {
        let condition = Expr::parse(parser);

        if !parser.current_token(&Token::Then) {
            panic!("expected: then, actual: {:?}", parser.current_token);
        }
        parser.step();

        if !parser.current_token(&Token::Newline) {
            panic!("expected: newline, actual: {:?}", parser.current_token);
        }
        parser.step();

        let resolution = Ast::parse(parser, Token::EndIf).items;

        if !parser.current_token(&Token::EndIf) {
            panic!("expected: endif, actual: {:?}", parser.current_token);
        }
        parser.step();

        StmtKind::If(condition, resolution)
    }

    /// Grammar: "while" (condition) "repeat" \n {statement}* "endwhile"
    pub fn parse_while(parser: &mut Parser) -> StmtKind {
        let condition = Expr::parse(parser);

        if !parser.current_token(&Token::Repeat) {
            panic!("expected: repeat, actual: {:?}", parser.current_token);
        }
        parser.step();

        if !parser.current_token(&Token::Newline) {
            panic!("expected: newline, actual: {:?}", parser.current_token);
        }
        parser.step();

        let resolution = Ast::parse(parser, Token::EndWhile).items;

        if !parser.current_token(&Token::EndWhile) {
            panic!("expected: endwhile, actual: {:?}", parser.current_token);
        }
        parser.step();

        StmtKind::While(condition, resolution)
    }

    /// Grammar: "let" (ident) "=" (expression)
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
        let input = "print \"hello, world!\"";

        let expected = vec![print_stmt!(string_literal!("hello, world!"))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn print_string_statement_newline() {
        let input = "print \"hello, world!\"\n";

        let expected = vec![print_stmt!(string_literal!("hello, world!"))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn print_int_statement() {
        let input = "print 123";

        let expected = vec![print_stmt!(int_literal!(123))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn print_int_statement_newline() {
        let input = "print 123\n";

        let expected = vec![print_stmt!(int_literal!(123))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn if_statement() {
        let input = "if \"TMP\" then\nprint \"hello, world!\" endif";

        let expected = vec![if_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn if_statement_newline() {
        let input = "if \"TMP\" then\nprint \"hello, world!\" endif\n";

        let expected = vec![if_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn while_statement() {
        let input = "while \"TMP\" repeat\nprint \"hello, world!\" endwhile";

        let expected = vec![while_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn while_statement_newline() {
        let input = "while \"TMP\" repeat\nprint \"hello, world!\" endwhile\n";

        let expected = vec![while_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn while_statement_nested_statements() {
        let input = "while \"TMP\" repeat\nprint \"hello, world!\"\nprint \"hello, world 2!\"\nprint \"hello, world 3!\"\nendwhile";

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
        let input = "while \"TMP\" repeat\nprint \"hello, world!\"\nif \"TMP\" then\nwhile \"TMP\" repeat\nprint \"hello, world 3!\"\nendwhile\nendif\nendwhile";

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
        let input = "let Ident = 123";

        let expected = vec![let_stmt!("Ident".to_string(), int_literal!(123))];

        check_abstract_tree(input, expected)
    }
}
