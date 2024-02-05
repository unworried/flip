use crate::{
    cache::Cache, diagnostics::DiagnosticBag, error::Result, lexer::Lexer, parser::Parser,
    resolver::Resolver, source::Source,
};

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
