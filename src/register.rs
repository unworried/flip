use std::fmt;
use std::str::FromStr;

pub enum RegisterFlag {
    Compare = 0x1,
}

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
            "C" => Ok(Self::C),
            "M" => Ok(Self::M),
            "SP" => Ok(Self::SP),
            "PC" => Ok(Self::PC),
            "BP" => Ok(Self::BP),
            "Flags" => Ok(Self::Flags),
            _ => Err(format!("unknown register: {}", s)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encoding() {
        assert_eq!(Register::A as u16, 0);
        assert_eq!(Register::B as u16, 1);
        assert_eq!(Register::C as u16, 2);
        assert_eq!(Register::M as u16, 3);
        assert_eq!(Register::SP as u16, 4);
        assert_eq!(Register::PC as u16, 5);
        assert_eq!(Register::BP as u16, 6);
        assert_eq!(Register::Flags as u16, 7);
    }
}
