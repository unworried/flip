use flipc::{
    diagnostics::DiagnosticBag, lexer::Lexer, parser::Parser, resolver::{scope::Scope, Resolver}, source::Source
};

fn main() -> Result<(), ()> {
    let line = r#"x = y; 
    let x = 4; let x = 4;
    while x == 1 {
        if x == 4 {
            x = 5;
            let y = 5;
        };
        let z = 6;
    };
    "#;

    let source = Source::new(line.to_string());
    let mut lexer = Lexer::new(line.to_string());
    let diagnostics = DiagnosticBag::new();
    let mut parser = Parser::new(&mut lexer, diagnostics.clone());

    let result = parser.parse();


    let mut resolver = Resolver::new(vec![Scope::default()], diagnostics.clone());
    resolver.resolve(&result);
    
    diagnostics.borrow().check(&source).map_err(|_| ())?;

    Ok(())
}
