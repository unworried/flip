use crate::ast::visitor::Walkable;
use std::ops::Deref;

use flipvm::op::{Instruction, Literal12Bit, Nibble, StackOp, TestOp};
use flipvm::Register;

use crate::ast::visitor::Visitor;
use crate::ast::{BinOp, Binary, Definition, Literal, LiteralKind};
use crate::Ast;

pub struct CodeGenerator {
    instructions: Vec<Instruction>,
}

impl CodeGenerator {
    pub fn run(ast: &Ast) -> Vec<Instruction> {
        let mut gen = CodeGenerator {
            instructions: Vec::new(),
        };

        ast.walk(&mut gen);
        gen.instructions
    }

    fn emit(&mut self, ins: Instruction) {
        self.instructions.push(ins);
    }
}

impl Visitor for CodeGenerator {
    fn visit_definition(&mut self, def: &Definition) {
        def.value.walk(self);

        self.emit(Instruction::Stack(Register::C, Register::SP, StackOp::Pop));
    }

    fn visit_binary(&mut self, bin: &Binary) {
        bin.right.walk(self);
        bin.left.walk(self);

        match bin.op {
            BinOp::Add => self.emit(Instruction::Stack(
                Register::Zero,
                Register::SP,
                StackOp::Add,
            )),
            BinOp::Sub => self.emit(Instruction::Stack(
                Register::Zero,
                Register::SP,
                StackOp::Sub,
            )),
            BinOp::LessThanEq => {
                self.instructions
                    .push(Instruction::Stack(Register::B, Register::SP, StackOp::Pop));
                self.instructions
                    .push(Instruction::Stack(Register::C, Register::SP, StackOp::Pop));
                self.instructions
                    .push(Instruction::Test(Register::B, Register::C, TestOp::Lte));
                self.emit(Instruction::Add(
                    Register::Zero,
                    Register::Zero,
                    Register::C,
                ));
                self.emit(Instruction::AddIf(
                    Register::C,
                    Register::Zero,
                    Nibble::new_checked(1).unwrap(),
                ));
                self.emit(Instruction::Stack(Register::C, Register::SP, StackOp::Push));
            }
            _ => unimplemented!("binop"),
        }
    }

    fn visit_literal(&mut self, lit: &Literal) {
        match &lit.kind {
            LiteralKind::Int(i) => {
                self.emit(Instruction::Imm(
                    Register::C,
                    Literal12Bit::new_checked(*i as u16).unwrap(),
                ));
                self.emit(Instruction::Stack(Register::C, Register::SP, StackOp::Push));
            }
            _ => unimplemented!("literal"),
        }
    }
}
