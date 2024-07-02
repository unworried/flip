use crate::memory::{Addressable, LinearMemory};

enum Register {
    A,
    B,
    C,
    SP,
    PC,
    BP,
    Flags,
}

pub struct Machine {
    registers: [u16; 8],
    memory: Box<dyn Addressable>,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            registers: [0; 8],
            memory: Box::new(LinearMemory::new(8 * 1024)),
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read2(pc).unwrap();
        self.registers[Register::PC as usize] = pc + 2;
        println!("{} @ {}", instruction, pc);
        Ok(())
    }
}
