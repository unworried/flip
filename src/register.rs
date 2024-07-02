#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    M,
    SP,
    PC,
    BP,
    Flags,
}

impl Register {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            x if x == Register::A as u8 => Some(Register::A),
            x if x == Register::B as u8 => Some(Register::B),
            x if x == Register::C as u8 => Some(Register::C),
            x if x == Register::M as u8 => Some(Register::M),
            x if x == Register::SP as u8 => Some(Register::SP),
            x if x == Register::PC as u8 => Some(Register::PC),
            x if x == Register::BP as u8 => Some(Register::BP),
            x if x == Register::Flags as u8 => Some(Register::Flags),
            _ => None,
        }
    }
}
