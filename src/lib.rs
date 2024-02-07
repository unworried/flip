//#![cfg_attr(not(test), no_std)]
//#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod diagnostics;
mod error;
mod escape_codes;
pub mod frontend;
mod lexer;
mod parser;
mod resolver;
mod source;
mod span;
