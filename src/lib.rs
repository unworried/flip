//#![cfg_attr(not(test), no_std)]
//#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub mod diagnostics;
mod error;
pub mod lexer;
pub mod parser;
pub mod resolver;
pub mod source;
mod span;
