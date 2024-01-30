use basic_compiler::lexer::{Lexer, Token};

fn main() {
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let mut tokenizer = Lexer::new(line);

            loop {
                let token = tokenizer.next_token();
                println!("{} ", token);
                if let Token::Eof = token {
                    break;
                }
            }
        }
    });
}
