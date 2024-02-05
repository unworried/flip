use flipc::{
    cache::Cache, diagnostics::DiagnosticBag, error::Result, lexer::Lexer, parser::Parser,
    resolver::Resolver, source::Source,
};

fn main() -> Result<()> {
    /*let line = r#"x ==:: y;
    let x = 4; let x = 4;
    while x == 1 {
        if x == 4 {
            x = 5;
            let y = 5;
        };
        let z = 6;
    };
    "#;*/

    let line = r#"let x ==:::::::       4 * (2+1);
    x = 4;
    "#;

    let diagnostics = DiagnosticBag::new();
    let cache = Cache::new(diagnostics.clone());

    let source = Source::new(line.to_string());
    let mut lexer = Lexer::new(line.to_string());
    let mut parser = Parser::new(&mut lexer, diagnostics.clone());

    let result = parser.parse();
    println!();
    println!("{}", result);

    let mut resolver = Resolver::new(cache);
    resolver.search(&result);

    diagnostics.borrow().check(&source)?;

    Ok(())
}
