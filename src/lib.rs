//#![cfg_attr(not(test), no_std)]
//#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

// Create frontend interal mod to remove need for pub modifiers
pub mod cache;
pub mod diagnostics;
pub mod error;
mod escape_codes;
pub mod lexer;
pub mod parser;
pub mod resolver;
pub mod source;
mod span;
