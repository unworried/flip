use basic_compiler::{lexer::Lexer, parser::Parser};

fn main() {
    //std::io::stdin().lines().for_each(|line| {
    //if let Ok(line) = line {
    //"PRINT \"hello, world!\"\nLET foo = 1001\nIF foo == 1001 THEN\nPRINT \"true\"\nENDIF"
    let line = r#"PRINT "hello, world!"
        IF "foo == 1001" THEN
            PRINT "true"
        ENDIF"#;

    /*
     * TODO: parser not recieving newline token?
     *
     */

    let mut tokenizer = Lexer::new(line.to_string());
    let mut parser = Parser::new(&mut tokenizer);

    let result = parser.parse();
    println!();
    println!("{}", result);
    //}
    //});
}
