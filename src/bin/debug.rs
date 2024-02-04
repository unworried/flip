use flipc::{
    diagnostics::DiagnosticBag,
    lexer::Lexer,
    parser::Parser,
    source::Source,
};

fn main() -> Result<(), ()> {
    let line = r#"let x = 4;
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

    let _result = parser.parse();


    diagnostics.borrow().check(&source).map_err(|_| ())?;

    Ok(())
}
