use std::collections::HashMap;

use crate::memory::{Addressable, LinearMemory};
use crate::op::{Instruction, StackOp, TestOp};
use crate::register::Flag;
use crate::Register;

type SignalFunction = fn(&mut Machine, arg: u16) -> Result<(), String>;

pub struct Machine {
    registers: [u16; 8],
    signal_handlers: HashMap<u8, SignalFunction>,
    flags: u16,
    pub halt: bool,
    // TODO: Change This
    pub memory: Box<dyn Addressable>,
}

impl Default for Machine {
    fn default() -> Self {
        Machine {
            registers: [0; 8],
            signal_handlers: HashMap::new(),
            flags: 0,
            halt: false,
            memory: Box::new(LinearMemory::new(8 * 1024)),
        }
    }
}

impl Machine {
    pub fn new(mem_words: usize) -> Machine {
        Machine {
            memory: Box::new(LinearMemory::new(2 * mem_words)),
            ..Self::default()
        }
    }

    pub fn reset(&mut self) {
        self.memory.zero_all();
        self.registers = [0; 8];
        self.halt = false;
        self.flags = 0;
    }

    pub fn state(&self) -> String {
        format!(
            "A: {} | B: {} | C: {} | M: {} | SP: {} | PC: {} | BP: {} | Flags: {:016b}",
            self.get_register(Register::A),
            self.get_register(Register::B),
            self.get_register(Register::C),
            self.get_register(Register::M),
            self.get_register(Register::SP),
            self.get_register(Register::PC),
            self.get_register(Register::BP),
            self.flags
        )
    }

    pub fn get_register(&self, r: Register) -> u16 {
        match r {
            Register::Zero => 0,
            _ => self.registers[r as usize],
        }
    }

    pub fn set_register(&mut self, r: Register, v: u16) {
        match r {
            Register::Zero => {}
            _ => self.registers[r as usize] = v,
        }
    }

    pub fn define_handler(&mut self, index: u8, f: SignalFunction) {
        self.signal_handlers.insert(index, f);
    }

    fn pop(&mut self, stack_pointer_register: Register) -> Result<u16, String> {
        let sp = self.get_register(stack_pointer_register) - 2;
        if let Some(v) = self.memory.read2(sp as u32) {
            self.set_register(stack_pointer_register, sp);
            Ok(v)
        } else {
            Err(format!("memory read fault @ 0x{:X}", sp))
        }
    }

    fn peek(&mut self, stack_pointer_register: Register) -> Result<u16, String> {
        let sp = self.get_register(stack_pointer_register) - 2;
        if let Some(v) = self.memory.read2(sp as u32) {
            Ok(v)
        } else {
            Err(format!("memory read fault @ 0x{:X}", sp))
        }
    }

    fn push(&mut self, stack_pointer_register: Register, v: u16) -> Result<(), String> {
        let sp = self.get_register(stack_pointer_register);
        if !self.memory.write2(sp as u32, v) {
            return Err(format!("memory write fault @ 0x{:X}", sp));
        }
        self.set_register(stack_pointer_register, sp + 2);
        Ok(())
    }

    fn set_flag(&mut self, flag: Flag, state: bool) {
        if state {
            self.flags |= flag as u16;
        } else {
            self.flags &= !(flag as u16);
        }
    }

    fn test_flag(&self, flag: Flag) -> bool {
        self.flags & (flag as u16) != 0
    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.get_register(Register::PC);
        let instruction = self
            .memory
            .read2(pc as u32)
            .ok_or_else(|| format!("pc read fail @ 0x{:X}", pc))?;
        self.set_register(Register::PC, pc + 2);

        let op = Instruction::try_from(instruction)?;
        println!("executing: {}", op);
        match op {
            Instruction::Imm(r, v) => {
                self.set_register(r, v);
                Ok(())
            }
            Instruction::Add(r0, r1, dst) => {
                let v0 = self.get_register(r0);
                let v1 = self.get_register(r1);
                self.set_register(dst, v0 + v1);
                Ok(())
            }
            Instruction::Sub(r0, r1, dst) => {
                let v0 = self.get_register(r0);
                let v1 = self.get_register(r1);
                self.set_register(dst, v0.wrapping_sub(v1));
                Ok(())
            }
            Instruction::AddImm(r, l) => {
                self.set_register(r, self.get_register(r) + (l.value as u16));
                Ok(())
            }
            Instruction::AddImmSigned(r, l) => {
                let register_raw = self.get_register(r);
                let imm_signed = l.as_signed();
                unsafe {
                    let register_signed: i16 = std::mem::transmute(register_raw);
                    self.set_register(
                        r,
                        std::mem::transmute(register_signed + (imm_signed as i16)),
                    );
                }
                Ok(())
            }
            Instruction::ShiftLeft(r0, r1, offset) => {
                let base = self.get_register(r0);
                self.set_register(r1, base << offset.value);
                Ok(())
            }
            Instruction::ShiftRightLogical(r0, r1, offset) => {
                let base = self.get_register(r0);
                self.set_register(r1, base >> offset.value);
                Ok(())
            }
            Instruction::ShiftRightArithmetic(r0, r1, offset) => {
                let base = self.get_register(r0);
                self.set_register(r1, base >> offset.value);
                Ok(())
            }
            Instruction::Load(r0, r1, r2) => {
                let base = self.get_register(r1);
                let page = self.get_register(r2);
                let addr = (base as u32) + ((page as u32) << 16);
                let w = self
                    .memory
                    .read2(addr)
                    .ok_or_else(|| format!("failed read word @ 0x{:X}", addr))?;
                self.set_register(r0, w);
                Ok(())
            }
            Instruction::Store(r0, r1, r2) => {
                let base = self.get_register(r0);
                let page = self.get_register(r1);
                let addr = (base as u32) + ((page as u32) << 16);
                match self.memory.write2(addr, self.get_register(r2)) {
                    true => Ok(()),
                    false => Err(format!(
                        "failed write word {} @ 0x{:X}",
                        self.get_register(r2),
                        addr
                    )),
                }
            }
            Instruction::Jump(b) => {
                self.set_register(Register::PC, self.get_register(Register::PC) + b.value);
                Ok(())
            }
            Instruction::Test(r0, r1, op) => {
                let v0 = self.get_register(r0);
                let v1 = self.get_register(r1);
                let res = match op {
                    TestOp::Eq => v0 == v1,
                    TestOp::Neq => v0 != v1,
                    TestOp::Lt => v0 < v1,
                    TestOp::Lte => v0 <= v1,
                    TestOp::Gt => v0 > v1,
                    TestOp::Gte => v0 >= v1,
                    TestOp::BothZero => v0 == 0 && v1 == 0,
                    TestOp::EitherNonZero => v0 != 0 || v1 != 0,
                    TestOp::BothNonZero => v0 != 0 && v1 != 0,
                };
                self.set_flag(Flag::Compare, res);
                Ok(())
            }
            Instruction::AddIf(r, offset) => {
                if self.test_flag(Flag::Compare) {
                    self.set_register(r, self.get_register(r) + 2 * (offset.value as u16));
                }
                Ok(())
            }
            Instruction::Stack(r, sp, op) => {
                match op {
                    StackOp::Push => {
                        let v = self.get_register(r);
                        self.push(sp, v)?
                    }
                    StackOp::Pop => {
                        let v = self.pop(sp)?;
                        self.set_register(r, v);
                    }
                    StackOp::Peek => {
                        let v = self.peek(sp)?;
                        self.set_register(r, v);
                    }
                    StackOp::Dup => {
                        let v = self.peek(sp)?;
                        self.push(sp, v)?;
                    }
                    StackOp::Swap => {
                        let a = self.pop(sp)?;
                        let b = self.pop(sp)?;
                        self.push(sp, a)?;
                        self.push(sp, b)?;
                    }
                    StackOp::Rotate => {
                        let a = self.pop(sp)?;
                        let b = self.pop(sp)?;
                        let c = self.pop(sp)?;
                        self.push(sp, a)?;
                        self.push(sp, c)?;
                        self.push(sp, b)?;
                    }
                    StackOp::Add => {
                        let a = self.pop(sp)?;
                        let b = self.pop(sp)?;
                        self.push(sp, a + b)?;
                    }
                    StackOp::Sub => {
                        let a = self.pop(sp)?;
                        let b = self.pop(sp)?;
                        self.push(sp, a - b)?;
                    }
                };
                Ok(())
            }
            Instruction::LoadStackOffset(target, sp, word_offset) => {
                let base = self.get_register(sp);
                let addr = base - ((word_offset.value as u16) * 2);
                self.set_register(
                    target,
                    self.memory.read2(addr as u32).ok_or_else(|| {
                        format!(
                            "invalid stack read: stack={}, offset={:X} @ {:X}",
                            sp, word_offset.value, addr
                        )
                    })?,
                );
                Ok(())
            }
            Instruction::System(Register::Zero, arg_register, signal) => {
                let sigfn = self
                    .signal_handlers
                    .get(&signal.value)
                    .ok_or_else(|| format!("unknown signal: 0x{:X}", signal.value))?;
                let arg = self.get_register(arg_register);
                sigfn(self, arg)
            }
            Instruction::System(sig, _, arg) => {
                let value = self.get_register(sig);
                if value > 0xff {
                    Err(format!("unknown signal: 0x{:X}, must be <= 0xff", value))
                } else {
                    let sigfn = self
                        .signal_handlers
                        .get(&(value as u8))
                        .ok_or_else(|| format!("unknown signal: 0x{:X}", value))?;
                    sigfn(self, arg.value as u16)
                }
            }
        }
    }
}
