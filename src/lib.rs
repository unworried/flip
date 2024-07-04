mod memory;
pub mod op;
mod register;
mod vm;

pub use register::{Flag, Register};
pub use vm::Machine;
