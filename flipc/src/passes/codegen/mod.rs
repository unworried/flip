use std::collections::HashMap;
use std::marker::PhantomData;

use crate::ast::visitor::Visitor;
use crate::ast::Program;
use crate::passes::SymbolTable;

use flipvm::op::{Instruction, Literal12Bit, Literal7Bit, Nibble, StackOp};
use flipvm::Register::{self, *};

use super::Pass;

mod generators;
#[cfg(test)]
mod tests;

pub struct CodeGenerator<'a> {
    inital_offset: u32,
    current_offset: u32,

    instructions: Vec<Instruction>,

    symbol_table: &'a SymbolTable,
    max_scope: usize,
    current_scope: usize,

    labels: HashMap<String, u32>,
    // TODO: Look into alternatives that arent O(n)
    //unlinked_references: HashMap<String, Vec<(usize, Register)>>, // O(1)
    unlinked_references: Vec<(usize, FutureType, Register, String)>,

    _phantom: PhantomData<&'a ()>,
}

#[repr(u8)]
enum FutureType {
    Imm,
    AddImm,
}

impl<'a> Pass for CodeGenerator<'a> {
    type Input = (&'a Program, &'a SymbolTable, u32);

    type Output = Vec<Instruction>;

    fn run((ast, symbol_table, inital_offset): Self::Input) -> Self::Output {
        let mut gen = CodeGenerator::new(symbol_table, inital_offset);

        gen.emit_init();
        gen.visit_program(ast);

        // TODO: Do i keep this? + error handling
        // Techincaly Instruction::Invalid will emit error
        assert!(gen.unlinked_references.is_empty());

        gen.instructions
    }
}

// FIXME: Impl Pass?
impl<'a> CodeGenerator<'a> {
    fn new(symbol_table: &'a SymbolTable, inital_offset: u32) -> Self {
        Self {
            inital_offset,
            current_offset: inital_offset,
            instructions: Vec::new(),
            symbol_table,
            max_scope: 0,
            current_scope: 0,
            labels: HashMap::new(),
            unlinked_references: Vec::new(),
            _phantom: PhantomData,
        }
    }

    fn emit_init(&mut self) {
        self.emit(Instruction::Imm(
            SP,
            Literal12Bit::new_checked(0x3ff).unwrap(),
        ));
        self.emit(Instruction::ShiftLeft(
            SP,
            SP,
            Nibble::new_checked(4).unwrap(),
        ));
        self.emit(Instruction::Stack(BP, SP, StackOp::Push));
        self.emit(Instruction::Stack(PC, SP, StackOp::Push));
        self.emit(Instruction::Add(SP, Zero, BP));
        self.imm_future(PC, "main".to_string());

        self.emit(Instruction::Imm(
            C,
            Literal12Bit::new_checked(0xf0).unwrap(),
        ));
        self.emit(Instruction::System(
            C,
            Zero,
            Nibble::new_checked(0).unwrap(),
        ));
    }

    fn emit(&mut self, ins: Instruction) {
        self.instructions.push(ins);

        self.current_offset += 2;
    }

    fn emit_compare(&mut self, comp: Instruction) {
        self.emit(Instruction::Stack(B, SP, StackOp::Pop));
        self.emit(Instruction::Stack(C, SP, StackOp::Pop));
        self.emit(comp);
        self.emit(Instruction::Add(Zero, Zero, C));
        self.emit(Instruction::AddIf(C, Zero, Nibble::new_checked(1).unwrap()));
        self.emit(Instruction::Stack(C, SP, StackOp::Push));
    }

    fn emit_function_exit(&mut self) {
        // Load return addr
        self.emit(Instruction::LoadStackOffset(
            C,
            BP,
            Nibble::new_checked(1).unwrap(),
        ));
        // Reload previous stack pointer ( BP - 2 )
        self.emit(Instruction::Add(BP, Zero, SP));
        self.emit(Instruction::AddImmSigned(
            SP,
            Literal7Bit::from_signed(-2).unwrap(),
        ));
        // Load previous base pointer
        self.emit(Instruction::LoadStackOffset(
            BP,
            BP,
            Nibble::new_checked(2).unwrap(),
        ));
        // Jump to return addr
        self.emit(Instruction::AddImm(C, Literal7Bit::new_checked(6).unwrap()));
        self.emit(Instruction::Add(C, Zero, PC));
    }

    fn imm_future(&mut self, r: Register, label: String) {
        match self.labels.get(&label) {
            Some(offset) => {
                let imm = Literal12Bit::new_checked(*offset as u16).unwrap();
                self.emit(Instruction::Imm(r, imm));
            }
            None => {
                self.unlinked_references
                    .push((self.instructions.len(), FutureType::Imm, r, label));

                self.emit(Instruction::Invalid); // Placeholder for labeled immediate
            }
        }
    }

    fn addimm_future(&mut self, r: Register, label: String) {
        match self.labels.get(&label) {
            Some(offset) => {
                let imm = Literal7Bit::new_checked(*offset as u8).unwrap();
                self.emit(Instruction::AddImm(r, imm));
            }
            None => {
                self.unlinked_references.push((
                    self.instructions.len(),
                    FutureType::AddImm,
                    r,
                    label,
                ));

                self.emit(Instruction::Invalid); // Placeholder for labeled immediate
            }
        }
    }

    fn define_label(&mut self, label: String) {
        self.define_label_offset(label, self.current_offset)
    }

    fn define_label_offset(&mut self, label: String, offset: u32) {
        self.labels.insert(label.clone(), offset);

        self.unlinked_references.retain(|(loc, ft, r, l)| {
            if *l == label {
                match ft {
                    FutureType::Imm => {
                        let imm = Literal12Bit::new_checked(offset as u16).unwrap();
                        self.instructions[*loc] = Instruction::Imm(*r, imm);
                    }
                    FutureType::AddImm => {
                        let imm = Literal7Bit::new_checked(offset as u8).unwrap();
                        self.instructions[*loc] = Instruction::AddImm(*r, imm);
                    }
                }
                false
            } else {
                true
            }
        });
    }

    fn enter_scope(&mut self) {
        self.max_scope += 1;
        self.current_scope = self.max_scope;
    }

    fn exit_scope(&mut self) {
        self.current_scope = self
            .symbol_table
            .lookup_scope(self.current_scope)
            .unwrap()
            .parent
            .unwrap();
    }
}
