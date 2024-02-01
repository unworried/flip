use basic_compiler::{lexer::Lexer, parser::Parser};

fn main() {
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let mut tokenizer = Lexer::new(line.to_string());
            let mut parser = Parser::new(&mut tokenizer);

            let result = parser.parse();
            println!();
            println!("{:#?}", result);
        }
    });
}
