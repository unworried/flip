use crate::Register;

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Push(u8),
    PopRegister(Register),
    AddStack,
    AddRegister(Register, Register),
    Signal(u8),
}

impl Instruction {
    fn encode_r1(r: Register) -> u16 {
        (r as u16) & 0xf << 8
    }
    fn encode_r2(r: Register) -> u16 {
        (r as u16) & 0xf << 12
    }

    fn encode_num(u: u8) -> u16 {
        (u as u16) << 8
    }

    fn encode_rs(r1: Register, r2: Register) -> u16 {
        Self::encode_r1(r1) | Self::encode_r2(r2)
    }

    pub fn encode_u16(&self) -> u16 {
        match self {
            Instruction::Nop => OpCode::Nop as u16,
            Instruction::Push(v) => OpCode::Push as u16 | Self::encode_num(*v),
            Instruction::PopRegister(r) => OpCode::PopRegister as u16 | Self::encode_r1(*r),
            Instruction::Signal(s) => OpCode::Signal as u16 | Self::encode_num(*s),
            Instruction::AddStack => OpCode::AddStack as u16,
            Instruction::AddRegister(r1, r2) => {
                OpCode::AddRegister as u16 | Self::encode_rs(*r1, *r2)
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Nop = 0x0,
    Push = 0x1,
    PopRegister = 0x2,
    Signal = 0x0f,
    AddStack = 0x10,
    AddRegister = 0x11,
}

impl OpCode {
    pub fn from_str(s: &str) -> Option<OpCode> {
        match s {
            "Nop" => Some(OpCode::Nop),
            "Push" => Some(OpCode::Push),
            "PopRegister" => Some(OpCode::PopRegister),
            "Signal" => Some(OpCode::Signal),
            "AddStack" => Some(OpCode::AddStack),
            "AddRegister" => Some(OpCode::AddRegister),
            _ => None,
        }
    }

    pub fn from_u8(b: u8) -> Option<OpCode> {
        match b {
            x if x == OpCode::Nop as u8 => Some(OpCode::Nop),
            x if x == OpCode::Push as u8 => Some(OpCode::Push),
            x if x == OpCode::PopRegister as u8 => Some(OpCode::PopRegister),
            x if x == OpCode::Signal as u8 => Some(OpCode::Signal),
            x if x == OpCode::AddStack as u8 => Some(OpCode::AddStack),
            x if x == OpCode::AddRegister as u8 => Some(OpCode::AddRegister),
            _ => None,
        }
    }
}
