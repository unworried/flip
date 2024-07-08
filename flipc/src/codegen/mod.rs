use std::cell::RefCell;
use std::collections::HashMap;

use crate::ast::visitor::Walkable;
use crate::passes::SymbolTable;

use flipvm::op::{Instruction, Literal12Bit, Literal7Bit, Nibble, StackOp, TestOp};
use flipvm::Register::{self, *};

use crate::ast::visitor::Visitor;
use crate::ast::{
    Assignment, BinOp, Binary, Definition, If, Literal, LiteralKind, Variable, While,
};
use crate::Ast;

pub struct CodeGenerator {
    inital_offset: u32,
    current_offset: u32,

    instructions: Vec<Instruction>,

    symbol_table: RefCell<SymbolTable>,
    scope_idx: usize,

    labels: HashMap<String, u32>,
    unlinked_references: Vec<(usize, Register, String)>,
}

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

    fn emit_unlinked(&mut self, r: Register, label: String) {
        self.unlinked_references
            .push((self.instructions.len(), r, label));

        self.emit(Instruction::Invalid); // Placeholder for labeled immediate
    }

    fn define_label(&mut self, label: String) {
        // TODO: Remove Clone
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

impl Visitor for CodeGenerator {
    fn visit_if(&mut self, if_expr: &If) {
        let block_id = format!("{}{}", if_expr.span.start, if_expr.span.end);
        let label_true = format!("lbl_{}_if_true", block_id);
        let label_out = format!("lbl_{}_if_out", block_id);
        if_expr.condition.walk(self);

        // test cond == false
        self.emit(Instruction::Stack(C, SP, StackOp::Pop));
        self.emit(Instruction::Test(C, Zero, TestOp::BothZero));
        self.emit(Instruction::AddIf(PC, PC, Nibble::new_checked(2).unwrap()));
        self.emit_unlinked(PC, label_true.clone());

        self.emit_unlinked(PC, label_out.clone());

        // if cond == true
        self.define_label(label_true);
        let scope_idx = self.enter_scope();
        if_expr.then.walk(self);
        self.exit_scope(scope_idx);

        self.emit_unlinked(PC, label_out.clone());
        self.define_label(label_out);
    }

    fn visit_while(&mut self, while_expr: &While) {
        unimplemented!("While");
        /*let scope_idx = self.enter_scope();
        while_expr.condition.walk(self);
        while_expr.then.walk(self);
        self.exit_scope(scope_idx);*/
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
            BinOp::Eq => {
                self.emit(Instruction::Stack(B, SP, StackOp::Pop));
                self.emit(Instruction::Stack(C, SP, StackOp::Pop));
                self.emit(Instruction::Test(B, C, TestOp::Eq));
                self.emit(Instruction::Add(Zero, Zero, C));
                self.emit(Instruction::AddIf(C, Zero, Nibble::new_checked(1).unwrap()));
                self.emit(Instruction::Stack(C, SP, StackOp::Push));
            }
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
