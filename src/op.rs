use std::fmt;
use std::str::FromStr;

use macros::VmInstruction;

use crate::Register;

// 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0
// OPERATOR        | ARG/S
//                 | 8bit literal
//                 | REG1  | REG2
#[derive(Debug, VmInstruction)]
pub enum Instruction {
    #[opcode(0x0)]
    Nop,
    #[opcode(0x1)]
    Push(u8),
    #[opcode(0x2)]
    PopRegister(Register),
    #[opcode(0x3)]
    PushRegister(Register),
    #[opcode(0x20)]
    AddStack,
    #[opcode(0x21)]
    AddRegister(Register, Register),
    #[opcode(0xf0)]
    Signal(u8),
}
