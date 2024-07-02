use std::collections::HashMap;

use crate::memory::{Addressable, LinearMemory};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    SP,
    PC,
    BP,
    Flags,
}

impl Register {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            x if x == Register::A as u8 => Some(Register::A),
            x if x == Register::B as u8 => Some(Register::B),
            x if x == Register::C as u8 => Some(Register::C),
            x if x == Register::SP as u8 => Some(Register::SP),
            x if x == Register::PC as u8 => Some(Register::PC),
            x if x == Register::BP as u8 => Some(Register::BP),
            x if x == Register::Flags as u8 => Some(Register::Flags),
            _ => None,
        }
    }
}

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

    fn from_u8(b: u8) -> Option<OpCode> {
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

// 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0
// OPERATOR        | ARG/S
//                 | 8bit literal
//                 | REG1  | REG2
fn parse_instruction(ins: u16) -> Result<Instruction, String> {
    let op = (ins & 0xff) as u8;
    match OpCode::from_u8(op).ok_or(format!("unknown op: {:X}", op))? {
        OpCode::Nop => Ok(Instruction::Nop),
        OpCode::Push => {
            let arg = parse_instruction_arg(ins);
            Ok(Instruction::Push(arg))
        }
        OpCode::PopRegister => {
            let reg = (ins & 0xf00) >> 8;
            Register::from_u8(reg as u8)
                .ok_or(format!("unknown register 0x{:X}", reg))
                .map(Instruction::PopRegister)
        }
        OpCode::AddStack => Ok(Instruction::AddStack),
        OpCode::AddRegister => {
            let reg1_raw = (ins & 0xf00) >> 8;
            let reg2_raw = (ins & 0xf000) >> 12;

            let reg1 = Register::from_u8(reg1_raw as u8)
                .ok_or(format!("unknown register 0x{:X}", reg1_raw))?;
            let reg2 = Register::from_u8(reg2_raw as u8)
                .ok_or(format!("unknown register 0x{:X}", reg2_raw))?;

            Ok(Instruction::AddRegister(reg1, reg2))
        }
        OpCode::Signal => {
            let arg = parse_instruction_arg(ins);
            Ok(Instruction::Signal(arg))
        }
    }
}

fn parse_instruction_arg(instruction: u16) -> u8 {
    ((instruction & 0xff00) >> 8) as u8
}

type SignalFunction = fn(&mut Machine) -> Result<(), String>;

pub struct Machine {
    registers: [u16; 8],
    signal_handlers: HashMap<u8, SignalFunction>,
    pub halt: bool,
    // TODO: Change This
    pub memory: Box<dyn Addressable>,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            registers: [0; 8],
            signal_handlers: HashMap::new(),
            halt: false,
            memory: Box::new(LinearMemory::new(8 * 1024)),
        }
    }

    pub fn get_register(&self, r: Register) -> u16 {
        self.registers[r as usize]
    }

    pub fn define_handler(&mut self, index: u8, f: SignalFunction) {
        self.signal_handlers.insert(index, f);
    }

    fn pop(&mut self) -> Result<u16, String> {
        let sp = self.registers[Register::SP as usize] - 2;
        if let Some(v) = self.memory.read2(sp) {
            self.registers[Register::SP as usize] -= 2;
            Ok(v)
        } else {
            Err(format!("memory read fault @ 0x{:X}", sp))
        }
    }

    fn push(&mut self, v: u16) -> Result<(), String> {
        let sp = self.registers[Register::SP as usize];
        if !self.memory.write2(sp, v) {
            return Err(format!("memory write fault @ 0x{:X}", sp));
        }
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self
            .memory
            .read2(pc)
            .ok_or(format!("pc read fail @ 0x{:X}", pc))?;
        self.registers[Register::PC as usize] = pc + 2;

        let op = parse_instruction(instruction)?;
        match op {
            Instruction::Nop => Ok(()),
            Instruction::Push(v) => self.push(v.into()),
            Instruction::PopRegister(r) => {
                let value = self.pop()?;
                self.registers[r as usize] = value;
                Ok(())
            }
            Instruction::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a + b)
            }
            Instruction::AddRegister(r1, r2) => {
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            }
            Instruction::Signal(signal) => {
                let sig_fn = self
                    .signal_handlers
                    .get(&signal)
                    .ok_or(format!("unknown signal: 0x{:X}", signal))?;
                sig_fn(self)
            }
        }
    }
}
