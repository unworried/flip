//! frontend.rs - Module for the compiler frontend wrapper. The frontend is responsible for taking
//! the input source code and converting it into an abstract syntax tree (AST) and then checking the
//! AST for syntax and semantic errors.
use crate::ast::Program;
use crate::diagnostics::DiagnosticBag;
use crate::error::{CompilerError, Result};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::passes::nameresolver::NameResolver;
use crate::passes::symbol_table::SymbolTableBuilder;
//use crate::passes::typechecker::TypeChecker;
use crate::passes::{Pass, SymbolTable};
use crate::source::Source;

pub fn check(input: &str) -> Result<(Program, SymbolTable)> {
    let diagnostics = DiagnosticBag::new();

    // Fix to make lexer take src
    let source = Source::new(input.to_string());
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(&mut lexer, diagnostics.clone());

    let mut root = parser.parse();

    //let nameres = NameResolver::new(diagnostics.clone());
    // let st = nameres.resolve(&mut root);
    let (mut st, mut ft) = SymbolTableBuilder::run((&root, diagnostics.clone()));
    NameResolver::run((&mut root, &mut st, &mut ft, diagnostics.clone()));
    //let st = TypeChecker::run((&mut root, st, &mut ft, diagnostics.clone()));
    eprintln!("{:#?}", st);

    eprintln!();
    eprintln!("{}", root);
    eprintln!();

    #[cfg(test)]
    assert!(diagnostics.borrow().is_empty());

    match diagnostics.borrow().check(&source) {
        Ok(_) => Ok(()),
        Err(CompilerError::DiagnosticWarning) => Ok(()), // TODO: Change maybe in future
        Err(e) => Err(e),
    }?;

    Ok((root, st))
}
