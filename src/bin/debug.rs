use flipc::{diagnostics::{display::DiagnosticsDisplay, DiagnosticBag}, lexer::Lexer, parser::Parser, source::Source};

fn main() {
    let line = r#"let x = 4;
    while x {
        if x {
            let y - 5;
        };
        let z = 6;
    };
    "#;

    let source = Source::new(line.to_string());
    let mut lexer = Lexer::new(line.to_string());
    let diagnostics = DiagnosticBag::new();
    let mut parser = Parser::new(&mut lexer, diagnostics.clone());

    let _result = parser.parse();
    //println!();
    //println!("{}", result);

    let diagnostics_binding = diagnostics.borrow();
    if !diagnostics_binding.diagnostics.is_empty() {
        let diagnostics_display =
            DiagnosticsDisplay::new(&source, &diagnostics_binding.diagnostics);
        diagnostics_display.print();
    }
}
