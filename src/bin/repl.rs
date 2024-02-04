use flipc::{diagnostics::DiagnosticBag, lexer::Lexer, parser::Parser, source::Source};

fn main() -> Result<(), ()> {
    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let source = Source::new(line.to_string());
            let mut lexer = Lexer::new(line.to_string());
            let diagnostics = DiagnosticBag::new();
            let mut parser = Parser::new(&mut lexer, diagnostics.clone());

            let result = parser.parse();
            println!();
            println!("{}", result);

            diagnostics.borrow().check(&source).ok();
        }
    });
    Ok(())
}
