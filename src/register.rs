use std::fmt;
use std::str::FromStr;

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

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::M => write!(f, "M"),
            Self::SP => write!(f, "SP"),
            Self::PC => write!(f, "PC"),
            Self::BP => write!(f, "BP"),
            Self::Flags => write!(f, "Flags"),
        }
    }
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::A),
            "M" => Ok(Self::B),
            "SP" => Ok(Self::SP),
            "PC" => Ok(Self::PC),
            "BP" => Ok(Self::BP),
            "Flags" => Ok(Self::Flags),
            _ => Err(format!("unknown register: {}", s)),
        }
    }
}
