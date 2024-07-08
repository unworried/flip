use std::cell::RefCell;
use std::collections::HashMap;

use crate::ast::visitor::Walkable;
use crate::passes::SymbolTable;

use flipvm::op::{Instruction, Literal12Bit, Nibble, StackOp};
use flipvm::Register::{self, *};

use crate::Ast;

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
    unlinked_references: Vec<(usize, Register, String)>,
}

// FIXME: Impl Pass?
impl CodeGenerator {
    pub fn run(ast: &Ast, symbol_table: SymbolTable, inital_offset: u32) -> Vec<Instruction> {
        let mut gen = CodeGenerator {
            inital_offset,
            current_offset: inital_offset,
            instructions: Vec::new(),
            symbol_table: RefCell::new(symbol_table),
            scope_idx: 0,
            labels: HashMap::new(),
            unlinked_references: Vec::new(),
        };

        ast.walk(&mut gen);
        gen.emit(Instruction::Imm(
            C,
            Literal12Bit::new_checked(0xf0).unwrap(),
        ));
        gen.emit(Instruction::System(
            C,
            Zero,
            Nibble::new_checked(0).unwrap(),
        ));

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

    fn emit_jump(&mut self, r: Register, label: String) {
        match self.labels.get(&label) {
            Some(offset) => {
                let imm = Literal12Bit::new_checked(*offset as u16).unwrap();
                self.emit(Instruction::Imm(r, imm));
            }
            None => {
                self.unlinked_references
                    .push((self.instructions.len(), r, label));

                self.emit(Instruction::Invalid); // Placeholder for labeled immediate
            }
        }
    }

    fn define_label(&mut self, label: String) {
        self.labels.insert(label.clone(), self.current_offset);

        self.unlinked_references.retain(|(loc, r, l)| {
            if *l == label {
                let imm = Literal12Bit::new_checked(self.current_offset as u16).unwrap();
                self.instructions[*loc] = Instruction::Imm(*r, imm);
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

        self.scope_idx
        //core::mem::replace(&mut self.scope_idx, 0)
    }

    fn exit_scope(&mut self, index: usize) {
        let previous_symbol_table = *self.symbol_table.borrow_mut().parent.take().unwrap();
        let new_scope = previous_symbol_table.lookup_scope(index).unwrap();
        self.symbol_table.swap(new_scope);
        self.symbol_table = RefCell::new(previous_symbol_table);
        self.scope_idx = index + 1;
    }
}
