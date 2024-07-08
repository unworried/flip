use crate::ast::visitor::Walkable;
use crate::passes::SymbolTable;

use flipvm::op::{Instruction, Literal12Bit, Literal7Bit, Nibble, StackOp};
use flipvm::Register::*;

use crate::ast::visitor::Visitor;
use crate::ast::{Assignment, BinOp, Binary, Definition, Literal, LiteralKind, Variable};
use crate::Ast;

pub struct CodeGenerator<'a> {
    instructions: Vec<Instruction>,
    symbol_table: &'a SymbolTable,
}

impl<'a> CodeGenerator<'a> {
    pub fn run(ast: &Ast, symbol_table: &'a SymbolTable) -> Vec<Instruction> {
        let mut gen = CodeGenerator {
            instructions: Vec::new(),
            symbol_table,
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
    }
}

impl Visitor for CodeGenerator<'_> {
    fn visit_definition(&mut self, def: &Definition) {
        def.value.walk(self);

        let info = self.symbol_table.lookup_variable(&def.pattern).unwrap();
        let addr = info.local_idx as u8 * 2;

        self.emit(Instruction::Stack(C, SP, StackOp::Pop));
        self.emit(Instruction::Add(BP, Zero, B));
        self.emit(Instruction::AddImm(
            B,
            Literal7Bit::new_checked(addr).unwrap(),
        ));
        self.emit(Instruction::StoreWord(B, Zero, C));
    }

    fn visit_assignment(&mut self, def: &Assignment) {
        def.value.walk(self);

        let info = self.symbol_table.lookup_variable(&def.pattern).unwrap();
        let addr = info.local_idx as u8 * 2;

        self.emit(Instruction::Stack(C, SP, StackOp::Pop));
        self.emit(Instruction::Add(BP, Zero, B));
        self.emit(Instruction::AddImm(
            B,
            Literal7Bit::new_checked(addr).unwrap(),
        ));
        self.emit(Instruction::StoreWord(B, Zero, C));
    }

    fn visit_variable(&mut self, var: &Variable) {
        let info = self.symbol_table.lookup_variable(var).unwrap();
        let addr = info.local_idx as u8 * 2;

        self.emit(Instruction::Add(BP, Zero, C));
        self.emit(Instruction::AddImm(
            C,
            Literal7Bit::new_checked(addr).unwrap(),
        ));
        self.emit(Instruction::LoadWord(C, C, Zero));
        self.emit(Instruction::Stack(C, SP, StackOp::Push));
    }

    fn visit_binary(&mut self, bin: &Binary) {
        bin.right.walk(self);
        bin.left.walk(self);

        match bin.op {
            BinOp::Add => self.emit(Instruction::Stack(Zero, SP, StackOp::Add)),
            BinOp::Sub => self.emit(Instruction::Stack(Zero, SP, StackOp::Sub)),
            _ => unimplemented!("binop"),
        }
    }

    fn visit_literal(&mut self, lit: &Literal) {
        match &lit.kind {
            LiteralKind::Int(i) => {
                self.emit(Instruction::Imm(
                    C,
                    Literal12Bit::new_checked(*i as u16).unwrap(),
                ));
                self.emit(Instruction::Stack(C, SP, StackOp::Push));
            }
            _ => unimplemented!("literal"),
        }
    }
}
