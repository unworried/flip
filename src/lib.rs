//#![cfg_attr(not(test), no_std)]
//#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod diagnostics;
mod error;
mod escape_codes;
mod evaluator;
pub mod frontend;
mod lexer;
mod nameresolver;
mod parser;
mod source;
mod span;
mod types;
