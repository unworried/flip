use flipc::{lexer::Lexer, parser::Parser};

use self::common::read_source_file;

mod common;

#[test]
fn precedence() {
    let src = read_source_file("precedence.fl");

    let mut lex = Lexer::new(src);
    let mut parser = Parser::new(&mut lex);
    
    let actual = parser.parse();
    println!("{:#?}", actual);
}
