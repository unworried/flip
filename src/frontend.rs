//! frontend.rs - Module for the compiler frontend wrapper. The frontend is responsible for taking
//! the input source code and converting it into an abstract syntax tree (AST) and then checking the
//! AST for syntax and semantic errors.
use crate::diagnostics::DiagnosticBag;
use crate::error::Result;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::source::Source;

pub fn check(input: &str) -> Result<()> {
    let diagnostics = DiagnosticBag::new();
    //let cache = Cache::new(diagnostics.clone());

    // Fix to make lexer take src
    let source = Source::new(input.to_string());
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(&mut lexer, diagnostics.clone());

    let result = parser.parse();
    println!();
    println!("{}", result);

    //let mut resolver = Resolver::new(&cache);
    //resolver.resolve(&result);

    diagnostics.borrow().check(&source)?;

    Ok(())
}
