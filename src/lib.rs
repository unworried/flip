//#![cfg_attr(not(test), no_std)]
//#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub mod diagnostics;
pub mod resolver;
mod error;
pub mod lexer;
pub mod parser;
pub mod source;
mod span;
