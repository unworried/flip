//#![cfg_attr(not(test), no_std)]
//#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod ast;
pub use ast::Ast;

mod codegen;
mod diagnostics;
mod error;
mod escape_codes;
mod evaluator;
pub mod frontend;
mod lexer;
mod parser;
mod passes;
mod source;
mod span;
