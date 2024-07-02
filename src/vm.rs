use std::collections::HashMap;

use crate::memory::{Addressable, LinearMemory};

#[repr(u8)]
#[derive(Debug)]
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

#[repr(u8)]
#[derive(Debug)]
enum Op {
    Nop,
    Push(u8),
    PopRegister(Register),
    AddStack,
    AddRegister(Register, Register),
    Signal(u8),
}

impl Op {
    pub fn value(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

// 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0
// OPERATOR        | ARG/S
//                 | 8bit literal
//                 | REG1  | REG2
fn parse_instruction(ins: u16) -> Result<Op, String> {
    let op = (ins & 0xff) as u8;
    match op {
        x if x == Op::Nop.value() => Ok(Op::Nop),
        x if x == Op::Push(0).value() => {
            let arg = parse_instruction_arg(ins);
            Ok(Op::Push(arg as u8))
        }
        x if x == Op::PopRegister(Register::A).value() => {
            let reg = (ins & 0xf00) >> 8;
            Register::from_u8(reg as u8)
                .ok_or(format!("unknown register 0x{:X}", reg))
                .map(Op::PopRegister)
        }
        x if x == Op::AddStack.value() => Ok(Op::AddStack),
        x if x == Op::Signal(0).value() => {
            let arg = parse_instruction_arg(ins);
            Ok(Op::Signal(arg))
        }
        _ => Err(format!("unknown operator 0x{:X}", op)),
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
            Op::Nop => Ok(()),
            Op::Push(v) => self.push(v.into()),
            Op::PopRegister(r) => {
                let value = self.pop()?;
                self.registers[r as usize] = value;
                Ok(())
            }
            Op::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a + b)
            }
            Op::AddRegister(r1, r2) => {
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            }
            Op::Signal(signal) => {
                let sig_fn = self
                    .signal_handlers
                    .get(&signal)
                    .ok_or(format!("unknown signal: 0x{:X}", signal))?;
                sig_fn(self)
            }
        }
    }
}
