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
    #[opcode(0x22)]
    SubStack,
    #[opcode(0x23)]
    SubRegister(Register, Register),

    #[opcode(0xf0)]
    Signal(u8),
}

pub enum InstructionParseError {
    NoContent,
    Fail(String),
}

impl From<String> for InstructionParseError {
    fn from(s: String) -> Self {
        Self::Fail(s)
    }
}

#[cfg(test)]
mod test {
    use super::Instruction::*;
    use super::*;

    #[test]
    fn test_encodings() {
        assert_eq!(SubStack.encode_u16(), 0x22);
        assert_eq!(Push(0x5).encode_u16(), 0x0501);
        assert_eq!(AddRegister(Register::B, Register::BP).encode_u16(), 0x6121);
        assert_eq!(Signal(0x05).encode_u16(), 0x05f0);
        assert_eq!(Nop.encode_u16(), 0x00);
        assert_eq!(PushRegister(Register::A).encode_u16(), 0x03);
    }
}
