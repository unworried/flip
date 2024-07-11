use std::cell::RefCell;
use std::collections::HashMap;

use crate::ast::visitor::Visitor;
use crate::ast::Program;
use crate::passes::SymbolTable;

use flipvm::op::{Instruction, Literal12Bit, Literal7Bit, Nibble, StackOp};
use flipvm::Register::{self, *};

mod generators;

pub struct CodeGenerator {
    inital_offset: u32,
    current_offset: u32,

    instructions: Vec<Instruction>,

    symbol_table: RefCell<SymbolTable>,
    scope_idx: usize,

    labels: HashMap<String, u32>,
    // TODO: Look into alternatives that arent O(n)
    //unlinked_references: HashMap<String, Vec<(usize, Register)>>, // O(1)
    unlinked_references: Vec<(usize, FutureType, Register, String)>,
}

#[repr(u8)]
enum FutureType {
    Imm,
    AddImm,
}

// FIXME: Impl Pass?
impl CodeGenerator {
    pub fn run(ast: &Program, symbol_table: SymbolTable, inital_offset: u32) -> Vec<Instruction> {
        let mut gen = CodeGenerator {
            inital_offset,
            current_offset: inital_offset,
            instructions: Vec::new(),
            symbol_table: RefCell::new(symbol_table),
            scope_idx: 0,
            labels: HashMap::new(),
            unlinked_references: Vec::new(),
        };

        gen.emit(Instruction::Imm(
            SP,
            Literal12Bit::new_checked(0x3ff).unwrap(),
        ));
        gen.emit(Instruction::ShiftLeft(
            SP,
            SP,
            Nibble::new_checked(4).unwrap(),
        ));
        gen.emit(Instruction::Stack(BP, SP, StackOp::Push));
        gen.emit(Instruction::Stack(PC, SP, StackOp::Push));
        gen.emit(Instruction::Add(SP, Zero, BP));
        gen.imm_future(PC, "main".to_string());

        gen.emit(Instruction::Imm(
            C,
            Literal12Bit::new_checked(0xf0).unwrap(),
        ));
        gen.emit(Instruction::System(
            C,
            Zero,
            Nibble::new_checked(0).unwrap(),
        ));

        gen.visit_program(ast);

        // TODO: Do i keep this? + error handling
        // Techincaly Instruction::Invalid will emit error
        assert!(gen.unlinked_references.is_empty());

        gen.instructions
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

    fn enter_scope(&mut self) -> usize {
        let previous_symbol_table = std::mem::take(&mut self.symbol_table);
        self.symbol_table.swap(
            previous_symbol_table
                .borrow()
                .lookup_scope(self.scope_idx)
                .unwrap(),
        );
        self.symbol_table.borrow_mut().parent = Some(Box::new(previous_symbol_table.into_inner()));

        //self.scope_idx
        core::mem::replace(&mut self.scope_idx, 0)
    }

    fn exit_scope(&mut self, index: usize) {
        let previous_symbol_table = *self.symbol_table.borrow_mut().parent.take().unwrap();
        let new_scope = previous_symbol_table.lookup_scope(index).unwrap();
        self.symbol_table.swap(new_scope);
        self.symbol_table = RefCell::new(previous_symbol_table);
        self.scope_idx = index + 1;
    }
}
