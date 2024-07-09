use crate::ast::visitor::Walkable;
use flipvm::op::{Instruction, Literal12Bit, Literal7Bit, Nibble, StackOp, TestOp};
use flipvm::Register::*;

use crate::ast::visitor::Visitor;
use crate::ast::{
    Assignment, BinOp, Binary, Definition, Function, If, Literal, LiteralKind, Unary, Variable,
    While,
};

use super::CodeGenerator;

impl Visitor for CodeGenerator {
    fn visit_function(&mut self, func: &Function) {
        self.define_label(func.pattern.name.clone());

        let local_off = format!("__internal_{}_local_offset", func.pattern.name);
        self.addimm_future(SP, local_off.clone());

        let scope_idx = self.enter_scope();
        func.body.walk(self);
        let local_count = self.symbol_table.borrow().local_count();
        self.exit_scope(scope_idx);

        // Return addr
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
        self.emit(Instruction::AddImm(C, Literal7Bit::new_checked(6).unwrap()));
        self.emit(Instruction::Add(C, Zero, PC));

        self.define_label_offset(local_off, local_count as u32 * 2);
    }

    fn visit_if(&mut self, if_expr: &If) {
        let block_id = format!("{}{}", if_expr.span.start, if_expr.span.end);
        let true_label = format!("lbl_{}_if_true", block_id);
        let out_label = format!("lbl_{}_if_out", block_id);
        if_expr.condition.walk(self);

        // test cond == false
        self.emit(Instruction::Stack(C, SP, StackOp::Pop));
        self.emit(Instruction::Test(C, Zero, TestOp::BothZero));
        self.emit(Instruction::AddIf(PC, PC, Nibble::new_checked(2).unwrap()));
        self.imm_future(PC, true_label.clone());

        self.imm_future(PC, out_label.clone());

        // if cond == true
        self.define_label(true_label);
        let scope_idx = self.enter_scope();
        if_expr.then.walk(self);
        self.exit_scope(scope_idx);

        self.imm_future(PC, out_label.clone());
        self.define_label(out_label);
    }

    fn visit_while(&mut self, while_expr: &While) {
        let block_id = format!("{}{}", while_expr.span.start, while_expr.span.end);
        let cond_label = format!("lbl_{}_while_cond", block_id);
        let out_label = format!("lbl_{}_while_out", block_id);
        self.define_label(cond_label.clone());
        while_expr.condition.walk(self);

        self.emit(Instruction::Stack(C, SP, StackOp::Pop));
        self.emit(Instruction::Test(C, Zero, TestOp::EitherNonZero));
        self.emit(Instruction::AddIf(PC, PC, Nibble::new_checked(2).unwrap()));
        self.imm_future(PC, out_label.clone());

        let scope_idx = self.enter_scope();
        while_expr.then.walk(self);
        self.exit_scope(scope_idx);
        self.imm_future(PC, cond_label);
        self.define_label(out_label);
    }

    fn visit_definition(&mut self, def: &Definition) {
        def.value.walk(self);

        let local_idx = self
            .symbol_table
            .borrow()
            .lookup_variable(&def.pattern)
            .unwrap()
            .local_idx;
        let addr = local_idx as u8 * 2;

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

        let local_idx = self
            .symbol_table
            .borrow()
            .lookup_variable(&def.pattern)
            .unwrap()
            .local_idx;
        let addr = local_idx as u8 * 2;

        self.emit(Instruction::Stack(C, SP, StackOp::Pop));
        self.emit(Instruction::Add(BP, Zero, B));
        self.emit(Instruction::AddImm(
            B,
            Literal7Bit::new_checked(addr).unwrap(),
        ));
        self.emit(Instruction::StoreWord(B, Zero, C));
    }

    fn visit_variable(&mut self, var: &Variable) {
        let local_idx = self
            .symbol_table
            .borrow()
            .lookup_variable(var)
            .unwrap()
            .local_idx;
        let addr = local_idx as u8 * 2;

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
            BinOp::Mul => unimplemented!("mul"),
            BinOp::Div => unimplemented!("div"),
            BinOp::Eq => self.emit_compare(Instruction::Test(B, C, TestOp::Eq)),
            BinOp::NotEq => self.emit_compare(Instruction::Test(B, C, TestOp::Neq)),
            BinOp::LessThan => self.emit_compare(Instruction::Test(B, C, TestOp::Lt)),
            BinOp::LessThanEq => self.emit_compare(Instruction::Test(B, C, TestOp::Lte)),
            BinOp::GreaterThan => self.emit_compare(Instruction::Test(B, C, TestOp::Gt)),
            BinOp::GreaterThanEq => self.emit_compare(Instruction::Test(B, C, TestOp::Gte)),
        }
    }

    fn visit_unary(&mut self, _un: &Unary) {
        unimplemented!("unary")
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
            LiteralKind::String(_) => unimplemented!("string literal"),
        }
    }
}
