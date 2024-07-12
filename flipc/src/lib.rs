//#![cfg_attr(not(test), no_std)]
//#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod ast;
pub use ast::Ast;

mod diagnostics;
mod error;
mod escape_codes;
mod evaluator;
pub mod frontend;
mod lexer;
mod parser;
mod passes;
pub use passes::CodeGenerator;
mod source;
mod span;
