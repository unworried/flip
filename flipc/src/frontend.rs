//! frontend.rs - Module for the compiler frontend wrapper. The frontend is responsible for taking
//! the input source code and converting it into an abstract syntax tree (AST) and then checking the
//! AST for syntax and semantic errors.
use std::io::stdout;
use std::io::Write;

use crate::codegen::CodeGenerator;
use crate::diagnostics::DiagnosticBag;
use crate::error::Result;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::passes::nameresolver::NameResolver;
use crate::passes::symbol_table::SymbolTableBuilder;
use crate::passes::Pass;
use crate::source::Source;

pub fn check(input: &str) -> Result<()> {
    let diagnostics = DiagnosticBag::new();

    // Fix to make lexer take src
    let source = Source::new(input.to_string());
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(&mut lexer, diagnostics.clone());

    let mut root = parser.parse();

    //let nameres = NameResolver::new(diagnostics.clone());
    // let st = nameres.resolve(&mut root);
    let st = SymbolTableBuilder::run((&root, diagnostics.clone()));
    let st = NameResolver::run((&mut root, st, diagnostics.clone()));
    eprintln!("{:#?}", st);

    eprintln!();
    eprintln!("{}", root);
    eprintln!();

    #[cfg(test)]
    assert!(diagnostics.borrow().diagnostics.is_empty());

    diagnostics.borrow().check(&source)?;

    let gen = CodeGenerator::run(&root, st, 0x0);
    let mut stdout = stdout().lock();
    let ins = gen
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    stdout
        .write_all(ins.as_bytes())
        .map_err(|e| format!("{}", e))
        .unwrap();

    Ok(())
}
