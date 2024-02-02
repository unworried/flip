use flipc::{lexer::Lexer, parser::Parser};

use self::common::read_source_file;

mod common;

#[test]
fn hello_world_bas() {
    let src = read_source_file("helloworld.fl");
    let mut lex = Lexer::new(src);
    let mut parser = Parser::new(&mut lex);

    let actual = parser.parse();
    println!("{:#?}", actual);
}
