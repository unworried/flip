use crate::cache::Cache;
use crate::diagnostics::DiagnosticBag;
use crate::error::Result;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::source::Source;

pub fn check(input: &str) -> Result<()> {
    let diagnostics = DiagnosticBag::new();
    let cache = Cache::new(diagnostics.clone());

    // Fix to make lexer take src
    let source = Source::new(input.to_string());
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(&mut lexer, diagnostics.clone());

    let result = parser.parse();
    println!();
    println!("{}", result);

    let mut resolver = Resolver::new(&cache);
    resolver.resolve(&result);

    diagnostics.borrow().check(&source)?;

    Ok(())
}
