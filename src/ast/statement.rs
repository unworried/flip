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

    /// Grammar: "if" (condition) "{" \n {statement}* "}"
    pub fn parse_if(parser: &mut Parser) -> StmtKind {
        let condition = Expr::parse(parser);

        if !parser.current_token(&Token::LBrace) {
            panic!("expected: '{{', actual: '{:?}'", parser.current_token);
        }
        parser.step();

        // Newline is optional May not need if allow newlines at start of file in ast root struct
        while parser.current_token(&Token::Newline) {
            parser.step();
        }

        let resolution = Ast::parse(parser, Token::RBrace).items;

        if !parser.current_token(&Token::RBrace) {
            panic!("expected: '}}', actual: '{:?}'", parser.current_token);
        }
        parser.step();

        StmtKind::If(condition, resolution)
    }

    /// Grammar: "while" (condition) "{" \n {statement}* "}"
    pub fn parse_while(parser: &mut Parser) -> StmtKind {
        let condition = Expr::parse(parser);

        if !parser.current_token(&Token::LBrace) {
            panic!("expected: '{{', actual: '{:?}'", parser.current_token);
        }
        parser.step();

        // Newline is optional May not need if allow newlines at start of file in ast root struct
        while parser.current_token(&Token::Newline) {
            parser.step();
        }

        let resolution = Ast::parse(parser, Token::RBrace).items;

        if !parser.current_token(&Token::RBrace) {
            panic!("expected: '}}', actual: '{:?}'", parser.current_token);
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

/*#[cfg(test)]
mod tests {
    use crate::{
        ast::{util::check_abstract_tree, Item}, if_stmt, int_literal, let_stmt, lexer::Lexer, parser::Parser, print_stmt,
        string_literal, while_stmt,
    };

    #[test]
    fn print_string_statement() {
        let input = "print \"hello, world!\";";

        let expected = vec![print_stmt!(string_literal!("hello, world!"))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn print_string_statement_newline() {
        let input = "print \"hello, world!\";\n";

        let expected = vec![print_stmt!(string_literal!("hello, world!"))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn print_int_statement() {
        let input = "print 123;";

        let expected = vec![print_stmt!(int_literal!(123))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn print_int_statement_newline() {
        let input = "print 123;\n";

        let expected = vec![print_stmt!(int_literal!(123))];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn if_statement() {
        let input = "if \"TMP\" { \nprint \"hello, world!\"; };";

        let expected = vec![if_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn if_statement_newline() {
        let input = "if \"TMP\" { \nprint \"hello, world!\"; }; \n";

        let expected = vec![if_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn while_statement() {
        let input = "while \"TMP\" { \nprint \"hello, world!\"; };";

        let expected = vec![while_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn while_statement_newline() {
        let input = "while \"TMP\" { \nprint \"hello, world!\"; }; \n";

        let expected = vec![while_stmt!(
            string_literal!("TMP"),
            vec![print_stmt!(string_literal!("hello, world!"))]
        )];

        check_abstract_tree(input, expected)
    }

    #[test]
    fn while_statement_nested_statements() {
        let input = "while \"TMP\" { \nprint \"hello, world!\"; \nprint \"hello, world 2!\"; \nprint \"hello, world 3!\"; \n };";

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
        let input = "while \"TMP\" { \nprint \"hello, world!\";\nif \"TMP\" { \nwhile \"TMP\" { \nprint \"hello, world 3!\";\n }; \n }; \n };";

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
        let input = "let Ident = 123;";

        let expected = vec![let_stmt!("Ident".to_string(), int_literal!(123))];

        check_abstract_tree(input, expected)
    }
}*/
